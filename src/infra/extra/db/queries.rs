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

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use domain::metadata;

    use super::*;

    #[sqlx::test(fixtures("seed"))]
    async fn all_query_works(pool: PgPool) {
        let result = AllQuery.exec(&pool).await;
        let extras = result.expect("All extras from fixtures");
        assert_eq!(extras.len(), 2);
    }

    #[sqlx::test(fixtures("seed"))]
    async fn create_query_works(pool: PgPool) {
        let extra = extra::Extra::new(
            extra::Name::new("Salad").expect("Salad is a valid extra name"),
            extra::Price::from_cents(250),
        );

        let result = CreateQuery { extra: &extra }.exec(&pool).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn delete_query_works(pool: PgPool) {
        let id = extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
            .expect("Valid extra id from fixtures");

        let result = DeleteQuery { id }.exec(&pool).await;
        let deleted = result.expect("Deleted row from fixtures");
        assert_eq!(deleted.id, id.uuid());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn find_many_query_works(pool: PgPool) {
        let ids = [
            "0190eaf5-c290-7443-b6a6-d22ce2a0fcb1",
            "0190eb06-f512-7302-a037-a223a9deb4e0",
        ]
        .iter()
        .map(|id| extra::Id::parse_str(id).expect("Valid extra id from fixtures"))
        .collect::<Vec<extra::Id>>();

        let result = FindManyQuery { ids: &ids }.exec(&pool).await;
        let found = result.expect("Found rows from fixtures");
        assert_eq!(found.len(), 2);
        assert!(found.iter().all(|f| ids.iter().any(|id| id.uuid() == f.id)));
    }

    #[sqlx::test(fixtures("seed"))]
    async fn find_query_works(pool: PgPool) {
        let id = extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
            .expect("Valid extra id from fixtures");

        let result = FindQuery { id }.exec(&pool).await;
        let found = result.expect("Found row from fixtures");
        assert_eq!(found.id, id.uuid());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn update_query_works(pool: PgPool) {
        let extra = extra::Extra::config(extra::Config {
            id: extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
                .expect("Valid extra id from fixtures"),
            name: extra::Name::new("Salad").expect("Salad is a valid extra name"),
            price: extra::Price::from_cents(250),
            metadata: metadata::Metadata::new(),
        });

        let result = UpdateQuery { extra: &extra }.exec(&pool).await;
        assert!(result.is_ok());

        let updated_model = FindQuery { id: extra.id() }
            .exec(&pool)
            .await
            .expect("Updated extra model");

        assert_eq!(updated_model.id, extra.id().uuid());
        assert_eq!(updated_model.name.as_str(), extra.name.as_str());
        assert_eq!(updated_model.price, extra.price.decimal());
    }
}
