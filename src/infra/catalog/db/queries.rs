use sqlx::PgExecutor;

use domain::catalog;

use crate::infra::catalog::CatalogWithProductsModel;

#[derive(Clone, Debug)]
pub(super) struct CreateQuery<'a> {
    pub(super) catalog: &'a catalog::Catalog,
}

impl<'a> CreateQuery<'a> {
    pub(super) async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let sql = include_str!("./sql/create.sql");
        sqlx::query(sql)
            .bind(self.catalog.id().uuid())
            .bind(self.catalog.name.as_str())
            .bind(
                self.catalog
                    .description
                    .as_ref()
                    .map(catalog::Description::as_str),
            )
            .bind(self.catalog.metadata.created_at())
            .bind(self.catalog.metadata.updated_at())
            .execute(exec)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct CountQuery;

impl CountQuery {
    pub(super) async fn exec(self, exec: impl PgExecutor<'_>) -> Result<u64, sqlx::Error> {
        let sql = include_str!("./sql/count.sql");
        let count: i64 = sqlx::query_scalar(sql).fetch_one(exec).await?;
        Ok(u64::try_from(count).unwrap_or_default())
    }
}

#[derive(Clone, Debug)]
pub(super) struct DeleteQuery {
    pub(super) id: catalog::Id,
}

impl DeleteQuery {
    pub(super) async fn exec(
        self,
        exec: impl PgExecutor<'_>,
    ) -> Result<CatalogWithProductsModel, sqlx::Error> {
        let sql = include_str!("./sql/delete.sql");
        sqlx::query_as(sql)
            .bind(self.id.uuid())
            .fetch_one(exec)
            .await
    }
}

#[derive(Clone, Debug)]
pub(super) struct FindQuery {
    pub(super) id: catalog::Id,
}

impl FindQuery {
    pub(super) async fn exec(
        self,
        exec: impl PgExecutor<'_>,
    ) -> Result<CatalogWithProductsModel, sqlx::Error> {
        let sql = include_str!("./sql/find.sql");
        sqlx::query_as(sql)
            .bind(self.id.uuid())
            .fetch_one(exec)
            .await
    }
}

#[derive(Clone, Debug)]
pub(super) struct ListQuery(pub(super) catalog::ListQuery);

impl ListQuery {
    pub(super) async fn exec(
        self,
        exec: impl PgExecutor<'_>,
    ) -> Result<Vec<CatalogWithProductsModel>, sqlx::Error> {
        let limit = u8::from(self.0.limit);
        let page = u32::from(self.0.page);
        let offset = page.saturating_sub(1) * u32::from(limit);

        let sql = include_str!("./sql/list.sql");
        sqlx::query_as(sql)
            .bind(i64::from(limit))
            .bind(i64::from(offset))
            .fetch_all(exec)
            .await
    }
}

#[derive(Clone, Debug)]
pub(super) struct UpdateQuery<'a> {
    pub(super) catalog: &'a catalog::Catalog,
}

impl<'a> UpdateQuery<'a> {
    pub(super) async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let sql = include_str!("./sql/update.sql");
        let result = sqlx::query(sql)
            .bind(self.catalog.name.as_str())
            .bind(
                self.catalog
                    .description
                    .as_ref()
                    .map(catalog::Description::as_str),
            )
            .bind(self.catalog.metadata.updated_at())
            .bind(self.catalog.id().uuid())
            .execute(exec)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use domain::metadata;
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test(fixtures("seed"))]
    async fn create_query_works(pool: PgPool) {
        let catalog = catalog::Catalog::new(
            catalog::Name::new("Vegetarian").expect("Valid catalog name"),
            Some(catalog::Description::new("Vegetarian foods").expect("Valid catalog description")),
        );

        let result = CreateQuery { catalog: &catalog }.exec(&pool).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn count_query_works(pool: PgPool) {
        let result = CountQuery.exec(&pool).await;
        assert_eq!(result.ok(), Some(2u64));
    }

    #[sqlx::test(fixtures("seed"))]
    async fn delete_query_works(pool: PgPool) {
        let id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let result = DeleteQuery { id }.exec(&pool).await;
        println!("{result:#?}");

        assert!(result.is_ok());

        let deleted_model = result.expect("Deleted catalog model");
        assert_eq!(deleted_model.id, id.uuid());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn find_query_works(pool: PgPool) {
        let id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let result = FindQuery { id }.exec(&pool).await;
        assert!(result.is_ok());

        let found_model = result.expect("Found catalog model");
        assert_eq!(found_model.id, id.uuid());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn list_query_works(pool: PgPool) {
        use std::num::{NonZeroU32, NonZeroU8};

        let query = catalog::ListQuery {
            page: NonZeroU32::new(1).unwrap(),
            limit: NonZeroU8::new(10).unwrap(),
        };

        let result = ListQuery(query).exec(&pool).await;
        assert!(result.is_ok());

        let list = result.expect("Catalog with products models");
        assert_eq!(list.len(), 2);
    }

    #[sqlx::test(fixtures("seed"))]
    async fn update_query_works(pool: PgPool) {
        let catalog = catalog::Catalog::config(catalog::Config {
            id: catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            name: catalog::Name::new("Burgers Updated").expect("Valid catalog name"),
            description: None,
            metadata: metadata::Metadata::new(),
        });

        let result = UpdateQuery { catalog: &catalog }.exec(&pool).await;
        assert!(result.is_ok());

        let updated_model = FindQuery { id: catalog.id() }
            .exec(&pool)
            .await
            .expect("Updated model");

        assert_eq!(updated_model.id, catalog.id().uuid());
        assert_eq!(updated_model.name.as_str(), catalog.name.as_str());
        assert_eq!(
            updated_model.description,
            catalog.description.map(|d| d.to_string())
        );
    }
}
