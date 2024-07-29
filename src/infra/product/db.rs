mod queries;

use sqlx::PgPool;

use domain::catalog;
use domain::product;

#[derive(Clone, Debug)]
pub struct PgProducts {
    pool: PgPool,
}

impl PgProducts {
    const PK: &'static str = "pk_product";
    const AK_NAME: &'static str = "ak_product_name";
    const FK_CATALOG_ID: &'static str = "fk_product_catalog_id";

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

    fn is_fk_catalog_id_error(err: &sqlx::Error) -> bool {
        err.as_database_error()
            .is_some_and(|db_err| db_err.constraint() == Some(Self::FK_CATALOG_ID))
    }
}

impl product::Repository for PgProducts {
    async fn create(&mut self, product: &product::Product) -> Result<(), product::Error> {
        let mut trx = self.pool.begin().await.map_err(product::Error::any)?;

        let create_query = queries::CreateQuery { product };
        create_query.exec(trx.as_mut()).await.map_err(|err| {
            if Self::is_pk_error(&err) {
                product::Error::id_conflict(product.id())
            } else if Self::is_ak_name_error(&err) {
                product::Error::name_conflict(product.name.clone())
            } else if Self::is_fk_catalog_id_error(&err) {
                product::Error::catalog_not_found(product.catalog_id())
            } else {
                product::Error::any(err)
            }
        })?;

        let bind_extras_query = queries::BindExtrasQuery {
            id: product.id(),
            extras: product.extras.as_slice(),
        };

        bind_extras_query
            .exec(trx.as_mut())
            .await
            .map_err(product::Error::any)?;

        trx.commit().await.map_err(product::Error::any)
    }

    async fn delete(
        &mut self,
        id: product::Id,
        catalog_id: catalog::Id,
    ) -> Result<product::Product, product::Error> {
        let query = queries::DeleteQuery { id, catalog_id };
        let model = query.exec(&self.pool).await.map_err(|err| match &err {
            sqlx::Error::RowNotFound => product::Error::id_not_found(id, catalog_id),
            _ => product::Error::any(err),
        })?;

        model.try_into_entity().map_err(product::Error::any)
    }

    async fn find(
        &self,
        id: product::Id,
        catalog_id: catalog::Id,
    ) -> Result<product::Product, product::Error> {
        let query = queries::FindQuery { id, catalog_id };
        let model = query.exec(&self.pool).await.map_err(|err| match &err {
            sqlx::Error::RowNotFound => product::Error::id_not_found(id, catalog_id),
            _ => product::Error::any(err),
        })?;

        model.try_into_entity().map_err(product::Error::any)
    }

    async fn update(&mut self, product: &product::Product) -> Result<(), product::Error> {
        let mut trx = self.pool.begin().await.map_err(product::Error::any)?;

        let update_query = queries::UpdateQuery { product };
        update_query.exec(trx.as_mut()).await.map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                product::Error::id_not_found(product.id(), product.catalog_id())
            } else if Self::is_ak_name_error(&err) {
                product::Error::name_conflict(product.name.clone())
            } else {
                product::Error::any(err)
            }
        })?;

        let bind_extras_query = queries::BindExtrasQuery {
            id: product.id(),
            extras: product.extras.as_slice(),
        };

        bind_extras_query
            .exec(trx.as_mut())
            .await
            .map_err(product::Error::any)?;

        let unbind_extras_query = queries::UnbindExtrasQuery {
            id: product.id(),
            extras: product.extras.as_slice(),
        };

        unbind_extras_query
            .exec(trx.as_mut())
            .await
            .map_err(product::Error::any)?;

        trx.commit().await.map_err(product::Error::any)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use domain::metadata;
    use domain::product::Repository;

    use super::*;

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_method_works(pool: PgPool) {
        let product = product::Product::new(
            catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            product::Name::new("Cheese Bacon").expect("Valid product name not in fixtures"),
            product::Price::from_cents(2100),
            product::Kind::Burger,
            product::Extras::default(),
        );

        let result = PgProducts::new(pool).create(&product).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_with_id_conflict(pool: PgPool) {
        use product::{ConflictKind, Error};

        let product = product::Product::config(product::Config {
            id: product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
                .expect("Valid product id from fixtures"),
            catalog_id: catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            name: product::Name::new("Cheese Bacon").expect("Valid product name not in fixtures"),
            price: product::Price::from_cents(2100),
            kind: product::Kind::Burger,
            extras: None,
            metadata: metadata::Metadata::new(),
        });

        let result = PgProducts::new(pool).create(&product).await;
        assert!(matches!(result, Err(Error::Conflict(ConflictKind::Id(id))) if id == product.id()));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_with_name_conflict(pool: PgPool) {
        use product::{ConflictKind, Error};

        let product = product::Product::new(
            catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            product::Name::new("Cheese Burger").expect("Valid product name from fixtures"),
            product::Price::from_cents(2100),
            product::Kind::Burger,
            product::Extras::default(),
        );

        let result = PgProducts::new(pool).create(&product).await;
        assert!(
            matches!(result, Err(Error::Conflict(ConflictKind::Name(name))) if name == product.name)
        );
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_with_catalog_not_found(pool: PgPool) {
        use product::{Error, NotFoundKind};

        let product = product::Product::new(
            catalog::Id::parse_str("0190fbba-d10a-73f0-bc03-3d1a44592ccf")
                .expect("Valid catalog id not in fixtures"),
            product::Name::new("Cheese Bacon").expect("Valid product name from fixtures"),
            product::Price::from_cents(2100),
            product::Kind::Burger,
            product::Extras::default(),
        );

        let result = PgProducts::new(pool).create(&product).await;
        assert!(
            matches!(result, Err(Error::NotFound(NotFoundKind::CatalogId(id))) if id == product.catalog_id())
        );
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn delete_method_works(pool: PgPool) {
        let id = product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
            .expect("Valid product id from fixtures");

        let catalog_id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let result = PgProducts::new(pool.clone()).delete(id, catalog_id).await;
        assert!(result.is_ok());

        let deleted = result.unwrap();
        assert_eq!(deleted.id(), id);
        assert_eq!(deleted.catalog_id(), catalog_id);

        let related_extras_count: i64 = sqlx::query_scalar(
            "select count(*) from product_extras as pe where pe.product_id = $1",
        )
        .bind(id.uuid())
        .fetch_one(&pool)
        .await
        .expect("Deleted product extras count");

        // check if all product relations with extras were deleted
        assert_eq!(related_extras_count, 0);
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn delete_with_not_found(pool: PgPool) {
        use product::{Error, NotFoundKind};

        let id = product::Id::parse_str("0190fbd1-8eea-7f43-942a-b356c1c631b6")
            .expect("Valid product id not in fixtures");

        let catalog_id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let result = PgProducts::new(pool).delete(id, catalog_id).await;
        assert!(matches!(result, Err(Error::NotFound(NotFoundKind::Id {
            id: err_id,
            catalog_id: err_catalog_id
        })) if err_id == id && err_catalog_id == catalog_id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_method_works(pool: PgPool) {
        let id = product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
            .expect("Valid product id from fixtures");

        let catalog_id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let result = PgProducts::new(pool).find(id, catalog_id).await;
        assert!(result.is_ok());

        let found = result.unwrap();
        assert_eq!(found.id(), id);
        assert_eq!(found.catalog_id(), catalog_id);
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_with_not_found(pool: PgPool) {
        use product::{Error, NotFoundKind};

        let id = product::Id::parse_str("0190fe6f-9240-77b2-8046-70af5be9c6bd")
            .expect("Valid product id not in fixtures");

        let catalog_id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let result = PgProducts::new(pool).find(id, catalog_id).await;
        assert!(matches!(result, Err(Error::NotFound(NotFoundKind::Id {
            id: err_id,
            catalog_id: err_catalog_id
        })) if err_id == id && err_catalog_id == catalog_id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn update_method_works(pool: PgPool) {
        let product = product::Product::config(product::Config {
            id: product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
                .expect("Valid product id from fixtures"),
            catalog_id: catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            name: product::Name::new("Cheese Bacon").expect("Valid product name not in fixtures"),
            price: product::Price::from_cents(2325),
            kind: product::Kind::Burger,
            extras: Some(product::Extras::default()),
            metadata: metadata::Metadata::new(),
        });

        let mut repository = PgProducts::new(pool);
        let result = repository.update(&product).await;
        assert!(result.is_ok());

        let updated = repository
            .find(product.id(), product.catalog_id())
            .await
            .unwrap();

        assert_eq!(updated.id(), product.id());
        assert_eq!(updated.catalog_id(), product.catalog_id());
        assert_eq!(updated.name, product.name);
        assert_eq!(updated.price, product.price);
        assert_eq!(updated.kind, product.kind);
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn update_with_not_found(pool: PgPool) {
        use product::{Error, NotFoundKind};

        let product = product::Product::config(product::Config {
            id: product::Id::parse_str("0190fe6f-9240-77b2-8046-70af5be9c6bd")
                .expect("Valid product id not in fixtures1"),
            catalog_id: catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            name: product::Name::new("Cheese Bacon").expect("Valid product name not in fixtures"),
            price: product::Price::from_cents(2325),
            kind: product::Kind::Burger,
            extras: Some(product::Extras::default()),
            metadata: metadata::Metadata::new(),
        });

        let result = PgProducts::new(pool).update(&product).await;
        assert!(matches!(result, Err(Error::NotFound(NotFoundKind::Id {
            id,
            catalog_id
        })) if id == product.id() && catalog_id == product.catalog_id()));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn update_with_name_conflict(pool: PgPool) {
        use product::{ConflictKind, Error};

        let product = product::Product::config(product::Config {
            id: product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
                .expect("Valid product id from fixtures"),
            catalog_id: catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            name: product::Name::new("Cheese Salad").expect("Valid product name not in fixtures"),
            price: product::Price::from_cents(2150),
            kind: product::Kind::Burger,
            extras: Some(product::Extras::default()),
            metadata: metadata::Metadata::new(),
        });

        let result = PgProducts::new(pool).update(&product).await;
        assert!(
            matches!(result, Err(Error::Conflict(ConflictKind::Name(name))) if name == product.name)
        );
    }
}
