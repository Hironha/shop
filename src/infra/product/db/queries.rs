use sqlx::PgExecutor;

use domain::catalog;
use domain::extra;
use domain::product;

use crate::infra::product::ProductModel;

// TODO: improve code organization and reduce memory memory allocation

#[derive(Clone, Debug)]
pub(super) struct BindExtrasQuery<'a> {
    pub(super) id: product::Id,
    pub(super) extras: &'a [extra::Extra],
}

impl<'a> BindExtrasQuery<'a> {
    pub(super) async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let extras_ids = self
            .extras
            .iter()
            .map(|extra| extra.id().uuid())
            .collect::<Vec<_>>();

        let product_ids = vec![self.id.uuid(); extras_ids.len()];

        let bind_extras_sql = include_str!("./sql/extras_bind.sql");
        sqlx::query(bind_extras_sql)
            .bind(product_ids)
            .bind(extras_ids)
            .execute(exec)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct CreateQuery<'a> {
    pub(super) product: &'a product::Product,
}

impl<'a> CreateQuery<'a> {
    pub(super) async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let sql = include_str!("./sql/create.sql");
        sqlx::query(sql)
            .bind(self.product.id().uuid())
            .bind(self.product.catalog_id().uuid())
            .bind(self.product.name().as_str())
            .bind(self.product.price().decimal())
            .bind(self.product.metadata().created_at())
            .bind(self.product.metadata().updated_at())
            .execute(exec)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct DeleteQuery {
    pub(super) id: product::Id,
    pub(super) catalog_id: catalog::Id,
}

impl DeleteQuery {
    pub async fn exec(self, exec: impl PgExecutor<'_>) -> Result<ProductModel, sqlx::Error> {
        let sql = include_str!("./sql/delete.sql");
        sqlx::query_as(sql)
            .bind(self.id.uuid())
            .bind(self.catalog_id.uuid())
            .fetch_one(exec)
            .await
    }
}

#[derive(Clone, Debug)]
pub(super) struct FindQuery {
    pub(super) id: product::Id,
    pub(super) catalog_id: catalog::Id,
}

impl FindQuery {
    pub async fn exec(self, exec: impl PgExecutor<'_>) -> Result<ProductModel, sqlx::Error> {
        let sql = include_str!("./sql/find.sql");
        sqlx::query_as(sql)
            .bind(self.id.uuid())
            .bind(self.catalog_id.uuid())
            .fetch_one(exec)
            .await
    }
}

#[derive(Clone, Debug)]
pub(super) struct UnbindExtrasQuery<'a> {
    pub(super) id: product::Id,
    pub(super) extras: &'a [extra::Extra],
}

impl<'a> UnbindExtrasQuery<'a> {
    pub async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let extras_ids = self
            .extras
            .iter()
            .map(|extra| extra.id().uuid())
            .collect::<Vec<_>>();

        let unbind_extras_sql = include_str!("./sql/extras_unbind.sql");
        sqlx::query(unbind_extras_sql)
            .bind(self.id.uuid())
            .bind(extras_ids)
            .execute(exec)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct UpdateQuery<'a> {
    pub(super) product: &'a product::Product,
}

impl<'a> UpdateQuery<'a> {
    pub async fn exec(self, exec: impl PgExecutor<'a>) -> Result<u64, sqlx::Error> {
        let update_sql = include_str!("./sql/update.sql");
        let result = sqlx::query(update_sql)
            .bind(self.product.name().as_str())
            .bind(self.product.price().decimal())
            .bind(self.product.metadata().updated_at())
            .bind(self.product.id().uuid())
            .bind(self.product.catalog_id().uuid())
            .execute(exec)
            .await?;

        Ok(result.rows_affected())
    }
}
