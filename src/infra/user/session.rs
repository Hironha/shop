use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

use domain::user;

use crate::app::user::service::session;

#[derive(Clone, Debug)]
pub struct PgSessions {
    pool: PgPool,
}

impl PgSessions {
    const DEFAULT_DURATION: Duration = Duration::hours(2);
    const EXP_DURATION_OFFSET: Duration = Duration::seconds(5);

    const PK: &'static str = "pk_session";

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn is_pk_error(err: &sqlx::Error) -> bool {
        err.as_database_error()
            .is_some_and(|e| e.constraint() == Some(Self::PK))
    }
}

impl session::Manager for PgSessions {
    async fn create(&mut self, user: &user::User) -> Result<(), session::Error> {
        let iss = OffsetDateTime::now_utc();
        let exp = iss.saturating_add(Self::DEFAULT_DURATION);

        let query = include_str!("./session/queries/create.sql");
        sqlx::query(query)
            .bind(user.id().uuid())
            .bind(iss)
            .bind(exp)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                if Self::is_pk_error(&err) {
                    session::Error::AlreadyExists(user.id())
                } else {
                    session::Error::any(err)
                }
            })?;

        Ok(())
    }

    async fn refresh(&mut self, id: user::Id) -> Result<(), session::Error> {
        let exp = OffsetDateTime::now_utc().saturating_add(Self::DEFAULT_DURATION);
        let query = include_str!("./session/queries/refresh.sql");
        let result = sqlx::query(query)
            .bind(exp)
            .bind(id.uuid())
            .execute(&self.pool)
            .await
            .map_err(session::Error::any)?;

        if result.rows_affected() == 0 {
            return Err(session::Error::NotFound(id));
        }

        Ok(())
    }

    async fn revoke(&mut self, id: user::Id) -> Result<(), session::Error> {
        let query = include_str!("./session/queries/delete.sql");
        let result = sqlx::query(query)
            .bind(id.uuid())
            .execute(&self.pool)
            .await
            .map_err(session::Error::any)?;

        if result.rows_affected() == 0 {
            return Err(session::Error::NotFound(id));
        }

        Ok(())
    }

    async fn validate(&self, id: user::Id) -> Result<bool, session::Error> {
        let query = include_str!("./session/queries/find_exp.sql");
        let exp: OffsetDateTime = sqlx::query_scalar(query)
            .bind(id.uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => session::Error::NotFound(id),
                _ => session::Error::any(err),
            })?;

        let exp = exp.saturating_sub(Self::EXP_DURATION_OFFSET);
        Ok(OffsetDateTime::now_utc() <= exp)
    }
}

#[cfg(test)]
mod tests {
    use domain::core::metadata::Metadata;

    use super::*;
    use crate::app::user::service::session::Manager;

    #[sqlx::test(fixtures("./session/fixtures/seed.sql"))]
    async fn create_method_works(pool: PgPool) {
        let user = user::User::config(user::UserConfig {
            id: user::Id::parse_str("01912f17-e118-7ec1-9aff-bd7d5636d1fc")
                .expect("Valid id from fixtures"),
            username: user::Username::try_new("marcus").expect("Valid username from fixtures"),
            email: user::Email::try_new("marcus@gmail.com").expect("Valid email from fixtures"),
            email_verified: true,
            metadata: Metadata::new(),
        });

        let mut pg_sessions = PgSessions::new(pool);
        let result = pg_sessions.create(&user).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("./session/fixtures/seed.sql"))]
    async fn refresh_method_works(pool: PgPool) {
        let id = user::Id::parse_str("019128e9-f215-7f81-b053-e8edd437df24")
            .expect("Valid id from fixtures");

        let mut pg_sessions = PgSessions::new(pool);
        let result = pg_sessions.refresh(id).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("./session/fixtures/seed.sql"))]
    async fn revoke_method_works(pool: PgPool) {
        let id = user::Id::parse_str("019128e9-f215-7f81-b053-e8edd437df24")
            .expect("Valid id from fixtures");

        let mut pg_sessions = PgSessions::new(pool);
        let result = pg_sessions.revoke(id).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("./session/fixtures/seed.sql"))]
    async fn validate_method_works(pool: PgPool) {
        let id = user::Id::parse_str("019128e9-f215-7f81-b053-e8edd437df24")
            .expect("Valid id from fixtures");

        let pg_sessions = PgSessions::new(pool);
        let result = pg_sessions.validate(id).await;
        assert_eq!(result.ok(), Some(true));
    }
}
