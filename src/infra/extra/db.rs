mod queries;

use sqlx::PgPool;

use domain::extra;

use super::model::ExtraModel;

#[derive(Clone, Debug)]
pub struct PgExtras {
    pool: PgPool,
}

impl PgExtras {
    const PK: &'static str = "pk_extra";
    const AK_NAME: &'static str = "ak_extra_name";

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

impl extra::Repository for PgExtras {
    async fn all(&self) -> Result<Vec<extra::Extra>, extra::Error> {
        let models = queries::AllQuery
            .exec(&self.pool)
            .await
            .map_err(extra::Error::any)?;

        models
            .into_iter()
            .map(ExtraModel::try_into_entity)
            .collect()
    }

    async fn create(&mut self, extra: &extra::Extra) -> Result<(), extra::Error> {
        queries::CreateQuery { extra }
            .exec(&self.pool)
            .await
            .map_err(|err| {
                if Self::is_pk_error(&err) {
                    extra::Error::id_conflict(extra.id())
                } else if Self::is_ak_name_error(&err) {
                    extra::Error::name_conflict(extra.name().clone())
                } else {
                    extra::Error::any(err)
                }
            })?;

        Ok(())
    }

    async fn delete(&mut self, id: extra::Id) -> Result<extra::Extra, extra::Error> {
        queries::DeleteQuery { id }
            .exec(&self.pool)
            .await
            .map_err(|err| match &err {
                sqlx::Error::RowNotFound => extra::Error::NotFound(id),
                _ => extra::Error::any(err),
            })
            .and_then(ExtraModel::try_into_entity)
    }

    async fn find(&self, id: extra::Id) -> Result<extra::Extra, extra::Error> {
        queries::FindQuery { id }
            .exec(&self.pool)
            .await
            .map_err(|err| match &err {
                sqlx::Error::RowNotFound => extra::Error::NotFound(id),
                _ => extra::Error::any(err),
            })
            .and_then(ExtraModel::try_into_entity)
    }

    async fn find_many(&self, ids: &[extra::Id]) -> Result<Vec<extra::Extra>, extra::Error> {
        let models = queries::FindManyQuery { ids }
            .exec(&self.pool)
            .await
            .map_err(extra::Error::any)?;

        let extras = models
            .into_iter()
            .map(ExtraModel::try_into_entity)
            .collect::<Result<Vec<_>, _>>()?;

        if let Some(id_not_found) = ids.iter().find(|id| !extras.iter().any(|e| e.id().eq(id))) {
            return Err(extra::Error::NotFound(*id_not_found));
        }

        Ok(extras)
    }

    async fn update(&mut self, extra: &extra::Extra) -> Result<(), extra::Error> {
        queries::UpdateQuery { extra }
            .exec(&self.pool)
            .await
            .map_err(|err| {
                if matches!(err, sqlx::Error::RowNotFound) {
                    extra::Error::NotFound(extra.id())
                } else if Self::is_ak_name_error(&err) {
                    extra::Error::name_conflict(extra.name().clone())
                } else {
                    extra::Error::any(err)
                }
            })?;

        Ok(())
    }
}
