use sqlx::PgExecutor;

use domain::extra;

use crate::infra::extra::ExtraModel;

#[derive(Clone, Debug)]
pub(super) struct AllQuery;

impl AllQuery {
    pub(super) async fn exec(
        self,
        exec: impl PgExecutor<'_>,
    ) -> Result<Vec<ExtraModel>, sqlx::Error> {
        let sql = include_str!("./sql/all.sql");
        sqlx::query_as(sql).fetch_all(exec).await
    }
}

#[derive(Clone, Debug)]
pub(super) struct CreateQuery<'a> {
    pub(super) extra: &'a extra::Extra,
}

impl<'a> CreateQuery<'a> {
    pub(super) async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let sql = include_str!("./sql/create.sql");
        sqlx::query(sql)
            .bind(self.extra.id().uuid())
            .bind(self.extra.name.as_str())
            .bind(self.extra.price.decimal())
            .bind(self.extra.metadata.created_at())
            .bind(self.extra.metadata.updated_at())
            .execute(exec)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct DeleteQuery {
    pub(super) id: extra::Id,
}

impl DeleteQuery {
    pub(super) async fn exec(self, exec: impl PgExecutor<'_>) -> Result<ExtraModel, sqlx::Error> {
        let sql = include_str!("./sql/delete.sql");
        sqlx::query_as(sql)
            .bind(self.id.uuid())
            .fetch_one(exec)
            .await
    }
}

#[derive(Clone, Debug)]
pub(super) struct FindManyQuery<'a> {
    pub(super) ids: &'a [extra::Id],
}

impl<'a> FindManyQuery<'a> {
    pub(super) async fn exec(
        self,
        exec: impl PgExecutor<'a>,
    ) -> Result<Vec<ExtraModel>, sqlx::Error> {
        let ids = self.ids.iter().map(extra::Id::uuid).collect::<Vec<_>>();
        let sql = include_str!("./sql/find_many.sql");
        sqlx::query_as(sql).bind(ids).fetch_all(exec).await
    }
}

#[derive(Clone, Debug)]
pub(super) struct FindQuery {
    pub(super) id: extra::Id,
}

impl FindQuery {
    pub(super) async fn exec(self, exec: impl PgExecutor<'_>) -> Result<ExtraModel, sqlx::Error> {
        let sql = include_str!("./sql/find.sql");
        sqlx::query_as(sql)
            .bind(self.id.uuid())
            .fetch_one(exec)
            .await
    }
}

#[derive(Clone, Debug)]
pub(super) struct UpdateQuery<'a> {
    pub(super) extra: &'a extra::Extra,
}

impl<'a> UpdateQuery<'a> {
    pub(super) async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let sql = include_str!("./sql/update.sql");
        let result = sqlx::query(sql)
            .bind(self.extra.name.as_str())
            .bind(self.extra.price.decimal())
            .bind(self.extra.metadata.updated_at())
            .bind(self.extra.id().uuid())
            .execute(exec)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
