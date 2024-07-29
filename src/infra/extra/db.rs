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
            .collect::<Result<Vec<_>, _>>()
            .map_err(extra::Error::any)
    }

    async fn create(&mut self, extra: &extra::Extra) -> Result<(), extra::Error> {
        let query = queries::CreateQuery { extra };
        query.exec(&self.pool).await.map_err(|err| {
            if Self::is_pk_error(&err) {
                extra::Error::id_conflict(extra.id())
            } else if Self::is_ak_name_error(&err) {
                extra::Error::name_conflict(extra.name.clone())
            } else {
                extra::Error::any(err)
            }
        })?;

        Ok(())
    }

    async fn delete(&mut self, id: extra::Id) -> Result<extra::Extra, extra::Error> {
        let query = queries::DeleteQuery { id };
        let model = query.exec(&self.pool).await.map_err(|err| match &err {
            sqlx::Error::RowNotFound => extra::Error::NotFound(id),
            _ => extra::Error::any(err),
        })?;

        model.try_into_entity().map_err(extra::Error::any)
    }

    async fn find(&self, id: extra::Id) -> Result<extra::Extra, extra::Error> {
        let query = queries::FindQuery { id };
        let model = query.exec(&self.pool).await.map_err(|err| match &err {
            sqlx::Error::RowNotFound => extra::Error::NotFound(id),
            _ => extra::Error::any(err),
        })?;

        model.try_into_entity().map_err(extra::Error::any)
    }

    async fn find_many(&self, ids: &[extra::Id]) -> Result<Vec<extra::Extra>, extra::Error> {
        let find_many_query = queries::FindManyQuery { ids };
        let models = find_many_query
            .exec(&self.pool)
            .await
            .map_err(extra::Error::any)?;

        let extras = models
            .into_iter()
            .map(ExtraModel::try_into_entity)
            .collect::<Result<Vec<_>, _>>()
            .map_err(extra::Error::any)?;

        if let Some(id_not_found) = ids.iter().find(|id| !extras.iter().any(|e| e.id().eq(id))) {
            return Err(extra::Error::NotFound(*id_not_found));
        }

        Ok(extras)
    }

    async fn update(&mut self, extra: &extra::Extra) -> Result<(), extra::Error> {
        let query = queries::UpdateQuery { extra };
        query.exec(&self.pool).await.map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                extra::Error::NotFound(extra.id())
            } else if Self::is_ak_name_error(&err) {
                extra::Error::name_conflict(extra.name.clone())
            } else {
                extra::Error::any(err)
            }
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use domain::extra::Repository;
    use domain::metadata;

    use super::*;

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn all_method_works(pool: PgPool) {
        let result = PgExtras::new(pool).all().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_method_works(pool: PgPool) {
        let extra = extra::Extra::new(
            extra::Name::new("Fork").expect("Valid extra name"),
            extra::Price::from_cents(150),
        );

        let result = PgExtras::new(pool).create(&extra).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_with_id_conflict(pool: PgPool) {
        use extra::{ConflictKind, Error};

        let id = extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
            .expect("Valid extra id from fixtures");

        let extra = extra::Extra::config(extra::Config {
            id,
            name: extra::Name::new("Fork").expect("Valid extra name"),
            price: extra::Price::from_cents(150),
            metadata: metadata::Metadata::new(),
        });

        let result = PgExtras::new(pool).create(&extra).await;
        assert!(matches!(result, Err(Error::Conflict(ConflictKind::Id(err_id))) if err_id == id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_with_name_conflict(pool: PgPool) {
        use extra::{ConflictKind, Error};

        let name = extra::Name::new("Cheese").expect("Valid extra name from fixtures");
        let extra = extra::Extra::new(name.clone(), extra::Price::from_cents(800));

        let result = PgExtras::new(pool).create(&extra).await;
        assert!(
            matches!(result, Err(Error::Conflict(ConflictKind::Name(err_name))) if err_name == name)
        );
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn delete_method_works(pool: PgPool) {
        let id = extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
            .expect("Valid extra id from fixtures");

        let result = PgExtras::new(pool).delete(id).await;
        assert!(matches!(result, Ok(extra) if extra.id() == id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn delete_with_not_found(pool: PgPool) {
        use extra::Error;

        let id = extra::Id::parse_str("0190f5d0-0209-7a43-9a57-e091e56493a4")
            .expect("Valid extra id not in fixtures");

        let result = PgExtras::new(pool).delete(id).await;
        assert!(matches!(result, Err(Error::NotFound(err_id)) if err_id == id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_method_works(pool: PgPool) {
        let id = extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
            .expect("Valid extra id from fixures");

        let result = PgExtras::new(pool).find(id).await;
        assert_eq!(result.ok().map(|extra| extra.id()), Some(id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_with_not_found(pool: PgPool) {
        use extra::Error;

        let id = extra::Id::parse_str("0190f5e1-5495-7391-9984-8997dbe367c6")
            .expect("Valid extra id not in fixtures");

        let result = PgExtras::new(pool).find(id).await;
        assert!(matches!(result, Err(Error::NotFound(err_id)) if err_id == id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_many_method_works(pool: PgPool) {
        let ids = vec![
            extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
                .expect("Valid extra id from fixtures"),
            extra::Id::parse_str("0190eb06-f512-7302-a037-a223a9deb4e0")
                .expect("Valid extra id from fixtures"),
        ];

        let result = PgExtras::new(pool).find_many(&ids).await;
        let extras = result.ok().unwrap_or_default();
        let extras_ids = extras.iter().map(extra::Extra::id).collect::<Vec<_>>();
        assert_eq!(extras_ids, ids);
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_many_with_not_found(pool: PgPool) {
        use extra::Error;

        let not_found_id = extra::Id::parse_str("0190f5e1-5495-7391-9984-8997dbe367c6")
            .expect("Valid extra id not in fixtures");

        let ids = vec![
            extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
                .expect("Valid extra id from fixtures"),
            extra::Id::parse_str("0190eb06-f512-7302-a037-a223a9deb4e0")
                .expect("Valid extra id from fixtures"),
            not_found_id,
        ];

        let result = PgExtras::new(pool).find_many(&ids).await;
        assert!(matches!(result, Err(Error::NotFound(err_id)) if err_id == not_found_id));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn update_method_works(pool: PgPool) {
        let extra = extra::Extra::config(extra::Config {
            id: extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
                .expect("Valid extra id from fixtures"),
            name: extra::Name::new("Cheddar").expect("Valid extra name"),
            price: extra::Price::from_cents(1200),
            metadata: metadata::Metadata::new(),
        });

        let result = PgExtras::new(pool).update(&extra).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn update_with_not_found(pool: PgPool) {
        use extra::Error;

        let extra = extra::Extra::config(extra::Config {
            id: extra::Id::parse_str("0190f5e1-5495-7391-9984-8997dbe367c6")
                .expect("Valid extra id not in fixtures"),
            name: extra::Name::new("Cheddar").expect("Valid extra name"),
            price: extra::Price::from_cents(1200),
            metadata: metadata::Metadata::new(),
        });

        let result = PgExtras::new(pool).update(&extra).await;
        assert!(matches!(result, Err(Error::NotFound(err_id)) if err_id == extra.id()));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn update_with_name_conflict(pool: PgPool) {
        use extra::{ConflictKind, Error};

        let extra = extra::Extra::config(extra::Config {
            id: extra::Id::parse_str("0190eaf5-c290-7443-b6a6-d22ce2a0fcb1")
                .expect("Valid extra id from fixtures"),
            name: extra::Name::new("Sauce").expect("Valid extra name from fixtures"),
            price: extra::Price::from_cents(600),
            metadata: metadata::Metadata::new(),
        });

        let result = PgExtras::new(pool).update(&extra).await;
        assert!(
            matches!(result, Err(Error::Conflict(ConflictKind::Name(err_name))) if err_name == extra.name)
        );
    }
}
