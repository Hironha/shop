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
            .bind(self.catalog.name().as_str())
            .bind(self.catalog.description().map(catalog::Description::as_str))
            .bind(self.catalog.metadata().created_at())
            .bind(self.catalog.metadata().updated_at())
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
pub(super) struct ListQuery {
    pub(super) page: u64,
    pub(super) limit: u16,
}

impl ListQuery {
    pub(super) async fn exec(
        self,
        exec: impl PgExecutor<'_>,
    ) -> Result<Vec<CatalogWithProductsModel>, sqlx::Error> {
        let limit = i64::from(self.limit);
        let offset =
            i64::try_from(self.page.saturating_sub(1) * u64::from(self.limit)).unwrap_or_default();

        let sql = include_str!("./sql/list.sql");
        sqlx::query_as(sql)
            .bind(limit)
            .bind(offset)
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
            .bind(self.catalog.name().as_str())
            .bind(self.catalog.description().map(catalog::Description::as_str))
            .bind(self.catalog.metadata().updated_at())
            .bind(self.catalog.id().uuid())
            .execute(exec)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
