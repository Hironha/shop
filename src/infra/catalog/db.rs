mod queries;

use sqlx::PgPool;

use domain::catalog;

use super::CatalogWithProductsModel;

#[derive(Clone, Debug)]
pub struct PgCatalogs {
    pool: PgPool,
}

impl PgCatalogs {
    const PK: &'static str = "pk_catalog";
    const AK_NAME: &'static str = "ak_catalog_name";

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn is_pk_error(err: &sqlx::Error) -> bool {
        err.as_database_error()
            .is_some_and(|db_err| db_err.constraint() == Some(Self::PK))
    }

    fn is_ak_name_error(err: &sqlx::Error) -> bool {
        err.as_database_error()
            .is_some_and(|db_err| db_err.constraint() == Some(Self::AK_NAME))
    }
}

impl catalog::Repository for PgCatalogs {
    async fn create(&mut self, catalog: &catalog::Catalog) -> Result<(), catalog::Error> {
        let query = queries::CreateQuery { catalog };
        query.exec(&self.pool).await.map_err(|err| {
            if Self::is_pk_error(&err) {
                catalog::Error::id_conflict(catalog.id())
            } else if Self::is_ak_name_error(&err) {
                let name = catalog.name.clone();
                catalog::Error::name_conflict(name)
            } else {
                catalog::Error::any(err)
            }
        })?;

        Ok(())
    }

    async fn delete(&self, id: catalog::Id) -> Result<catalog::ProductCatalog, catalog::Error> {
        let query = queries::DeleteQuery { id };
        let model = query.exec(&self.pool).await.map_err(|err| match err {
            sqlx::Error::RowNotFound => catalog::Error::id_not_found(id),
            _ => catalog::Error::any(err),
        })?;

        model.try_into_entity().map_err(catalog::Error::any)
    }

    async fn find(&self, id: catalog::Id) -> Result<catalog::ProductCatalog, catalog::Error> {
        let query = queries::FindQuery { id };
        let model = query.exec(&self.pool).await.map_err(|err| match err {
            sqlx::Error::RowNotFound => catalog::Error::id_not_found(id),
            _ => catalog::Error::any(err),
        })?;

        model.try_into_entity().map_err(catalog::Error::any)
    }

    async fn list(&self, query: catalog::ListQuery) -> Result<catalog::Pagination, catalog::Error> {
        let count = queries::CountQuery
            .exec(&self.pool)
            .await
            .map_err(catalog::Error::any)?;

        let list_query = queries::ListQuery(query.clone());
        let models = list_query
            .exec(&self.pool)
            .await
            .map_err(catalog::Error::any)?;

        let catalogs = models
            .into_iter()
            .map(CatalogWithProductsModel::try_into_entity)
            .collect::<Result<Vec<_>, _>>()
            .map_err(catalog::Error::any)?;

        Ok(catalog::Pagination {
            count,
            page: query.page,
            limit: query.limit,
            items: catalogs,
        })
    }

    async fn update(&mut self, catalog: &catalog::Catalog) -> Result<(), catalog::Error> {
        let query = queries::UpdateQuery { catalog };
        query.exec(&self.pool).await.map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                catalog::Error::id_not_found(catalog.id())
            } else if Self::is_ak_name_error(&err) {
                catalog::Error::name_conflict(catalog.name.clone())
            } else {
                catalog::Error::any(err)
            }
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use domain::catalog::Repository;
    use domain::core::metadata;

    use super::*;

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_method_works(pool: PgPool) {
        let catalog = catalog::Catalog::new(
            catalog::Name::new("Vegetarian").expect("Valid catalog name not in fixtures"),
            None,
        );

        let result = PgCatalogs::new(pool).create(&catalog).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_with_id_conflict(pool: PgPool) {
        use catalog::{ConflictKind, Error};

        let catalog = catalog::Catalog::config(catalog::CatalogConfig {
            id: catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            name: catalog::Name::new("Vegetarian").expect("Valid catalog name not in fixtures"),
            description: None,
            metadata: metadata::Metadata::new(),
        });

        let result = PgCatalogs::new(pool).create(&catalog).await;
        assert!(
            matches!(result, Err(Error::Conflict(ConflictKind::Id(err_id))) if err_id == catalog.id())
        );
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_with_name_conflict(pool: PgPool) {
        use catalog::{ConflictKind, Error};

        let catalog = catalog::Catalog::config(catalog::CatalogConfig {
            id: catalog::Id::parse_str("0190fa37-f4e0-7de0-86a2-7a0d60563f34")
                .expect("Valid catalog id not in fixtures"),
            name: catalog::Name::new("Burgers").expect("Valid catalog name from fixtures"),
            description: None,
            metadata: metadata::Metadata::new(),
        });

        let result = PgCatalogs::new(pool).create(&catalog).await;
        assert!(
            matches!(result, Err(Error::Conflict(ConflictKind::Name(err_name))) if err_name == catalog.name)
        );
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn delete_method_works(pool: PgPool) {
        let id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let result = PgCatalogs::new(pool).delete(id).await;
        assert!(matches!(result, Ok(cp) if cp.catalog.id() == id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn delete_with_not_found(pool: PgPool) {
        use catalog::{Error, NotFoundKind};

        let id = catalog::Id::parse_str("0190fa41-41dc-77f3-a1f1-267b4a38c8f3")
            .expect("Valid catalog id not in fixtures");

        let result = PgCatalogs::new(pool).delete(id).await;
        assert!(matches!(result, Err(Error::NotFound(NotFoundKind::Id(err_id))) if err_id == id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_method_works(pool: PgPool) {
        let id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let result = PgCatalogs::new(pool).find(id).await;
        assert!(matches!(&result, Ok(cp) if cp.catalog.id() == id));
        assert_eq!(result.ok().map(|cp| cp.products.len()), Some(1));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_with_not_found(pool: PgPool) {
        use catalog::{Error, NotFoundKind};

        let id = catalog::Id::parse_str("0190fa41-41dc-77f3-a1f1-267b4a38c8f3")
            .expect("Valid catalog id not in fixtures");

        let result = PgCatalogs::new(pool).find(id).await;
        assert!(matches!(result, Err(Error::NotFound(NotFoundKind::Id(err_id))) if err_id == id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn list_method_works(pool: PgPool) {
        use std::num::{NonZeroU32, NonZeroU8};

        let query = catalog::ListQuery {
            limit: NonZeroU8::new(10).unwrap(),
            page: NonZeroU32::new(1).unwrap(),
        };

        let result = PgCatalogs::new(pool).list(query.clone()).await;
        assert!(result.is_ok());

        let pagination = result.expect("Paginated catalog list");
        assert_eq!(pagination.count, 2);
        assert_eq!(pagination.items.len(), 2);
        assert_eq!(pagination.page, query.page);
        assert_eq!(pagination.limit, query.limit);
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn update_method_works(pool: PgPool) {
        let catalog = catalog::Catalog::config(catalog::CatalogConfig {
            id: catalog::Id::parse_str("0190ec30-7e38-75c0-a207-13c52449957d")
                .expect("Valid catalog id from fixtures"),
            name: catalog::Name::new("Vegetarian").expect("Valid catalog name"),
            description: Some(
                catalog::Description::new("Delicous vegetarian meals")
                    .expect("Valid catalog description"),
            ),
            metadata: metadata::Metadata::new(),
        });

        let mut repository = PgCatalogs::new(pool);
        let result = repository.update(&catalog).await;
        assert!(result.is_ok());

        let updated_catalog = repository
            .find(catalog.id())
            .await
            .map(|cp| cp.catalog)
            .expect("Updated catalog with products");

        assert_eq!(updated_catalog.id(), catalog.id());
        assert_eq!(updated_catalog.name, catalog.name);
        assert_eq!(updated_catalog.description, catalog.description);
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn update_with_not_found(pool: PgPool) {
        use catalog::{Error, NotFoundKind};

        let catalog = catalog::Catalog::config(catalog::CatalogConfig {
            id: catalog::Id::parse_str("0190fad2-0eab-7753-9b8c-583b79e7e51e")
                .expect("Valid catalog id not in fixtures"),
            name: catalog::Name::new("Vegetarian").expect("Valid catalog name"),
            description: Some(
                catalog::Description::new("Delicous vegetarian meals")
                    .expect("Valid catalog description"),
            ),
            metadata: metadata::Metadata::new(),
        });

        let result = PgCatalogs::new(pool).update(&catalog).await;
        assert!(matches!(result, Err(Error::NotFound(NotFoundKind::Id(id))) if id == catalog.id()));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn update_with_name_conflict(pool: PgPool) {
        use catalog::{ConflictKind, Error};

        let catalog = catalog::Catalog::config(catalog::CatalogConfig {
            id: catalog::Id::parse_str("0190ec30-7e38-75c0-a207-13c52449957d")
                .expect("Valid catalog id from fixtures"),
            name: catalog::Name::new("Burgers").expect("Valid catalog name from fixtures"),
            description: None,
            metadata: metadata::Metadata::new(),
        });

        let result = PgCatalogs::new(pool).update(&catalog).await;
        assert!(
            matches!(result, Err(Error::Conflict(ConflictKind::Name(name))) if name == catalog.name)
        );
    }
}
