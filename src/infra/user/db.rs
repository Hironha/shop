pub(super) mod model;

use domain::user;
use domain::user::password::Password;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct PgUsers {
    pool: PgPool,
}

impl PgUsers {
    const PK: &'static str = "pk_user";
    const AK_EMAIL: &'static str = "ak_user_email";

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn is_pk_error(err: &sqlx::Error) -> bool {
        err.as_database_error()
            .is_some_and(|e| e.constraint() == Some(Self::PK))
    }

    fn is_ak_email_error(err: &sqlx::Error) -> bool {
        err.as_database_error()
            .is_some_and(|e| e.constraint() == Some(Self::AK_EMAIL))
    }
}

impl user::Repository for PgUsers {
    async fn create(&mut self, user: &user::User, password: &Password) -> Result<(), user::Error> {
        let query = include_str!("./db/queries/create.sql");
        sqlx::query(query)
            .bind(user.id().uuid())
            .bind(user.username.as_str())
            .bind(user.email.as_str())
            .bind(user.is_email_verified())
            .bind(password.as_str())
            .bind(user.metadata.created_at())
            .bind(user.metadata.updated_at())
            .execute(&self.pool)
            .await
            .map_err(|err| {
                if Self::is_pk_error(&err) {
                    user::Error::id_conflict(user.id())
                } else if Self::is_ak_email_error(&err) {
                    user::Error::email_conflict(user.email.clone())
                } else {
                    user::Error::any(err)
                }
            })?;

        Ok(())
    }

    async fn find_by_email(&self, email: &user::Email) -> Result<user::User, user::Error> {
        let query = include_str!("./db/queries/find_by_email.sql");
        let user: model::UserModel = sqlx::query_as(query)
            .bind(email.as_str())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => user::Error::email_not_found(email.clone()),
                _ => user::Error::any(err),
            })?;

        user.try_into_entity().map_err(user::Error::any)
    }

    async fn find_password_by_email(&self, email: &user::Email) -> Result<String, user::Error> {
        let query = include_str!("./db/queries/find_password_by_email.sql");
        let password: String = sqlx::query_scalar(query)
            .bind(email.as_str())
            .fetch_one(&self.pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => user::Error::email_not_found(email.clone()),
                _ => user::Error::any(err),
            })?;

        Ok(password)
    }
}

#[cfg(test)]
mod tests {
    use domain::core::metadata::Metadata;
    use domain::user::Repository;

    use super::*;
    use crate::infra::Argon2Encrypter;

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_method_works(pool: PgPool) {
        let password = Password::new("Test123@", &Argon2Encrypter::new());
        let user = user::User::new(
            user::Username::try_new("Admin").expect("Valid username not in fixtures"),
            user::Email::try_new("admin@admin.com").expect("Valid email not in fixtures"),
        );

        let mut pg_users = PgUsers::new(pool);
        let result = pg_users.create(&user, &password).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_with_id_conflict(pool: PgPool) {
        use domain::user::{ConflictKind, Error};

        let password = Password::new("Test123@", &Argon2Encrypter::new());
        let user = user::User::config(user::UserConfig {
            id: user::Id::parse_str("019128e9-f215-7f81-b053-e8edd437df24")
                .expect("Valid id from fixtures"),
            username: user::Username::try_new("Admin").expect("Valid username not in fixtures"),
            email: user::Email::try_new("test@test.com").expect("Valid email not in fixtures"),
            email_verified: true,
            metadata: Metadata::new(),
        });

        let mut pg_users = PgUsers::new(pool);
        let result = pg_users.create(&user, &password).await;
        assert!(matches!(result, Err(Error::Conflict(ConflictKind::Id(id))) if id == user.id()));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn create_with_email_conflict(pool: PgPool) {
        use domain::user::{ConflictKind, Error};

        let password = Password::new("Test123@", &Argon2Encrypter::new());
        let user = user::User::config(user::UserConfig {
            id: user::Id::parse_str("019128e9-f215-7f81-b053-e8edd437df24")
                .expect("Valid id from fixtures"),
            username: user::Username::try_new("Admin").expect("Valid username not in fixtures"),
            email: user::Email::try_new("test@test.com").expect("Valid email not in fixtures"),
            email_verified: true,
            metadata: Metadata::new(),
        });

        let mut pg_users = PgUsers::new(pool);
        let result = pg_users.create(&user, &password).await;
        assert!(matches!(result, Err(Error::Conflict(ConflictKind::Id(id))) if id == user.id()));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_by_email_method_woks(pool: PgPool) {
        let email = user::Email::try_new("test@test.com").expect("Valid email from fixtures");

        let pg_users = PgUsers::new(pool);
        let result = pg_users.find_by_email(&email).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.email, email);
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_by_email_with_not_found(pool: PgPool) {
        use domain::user::{Error, NotFoundKind};

        let email = user::Email::try_new("jeff@gmail.com").expect("Valid email not in fixtures");

        let pg_users = PgUsers::new(pool);
        let result = pg_users.find_by_email(&email).await;
        assert!(matches!(result, Err(Error::NotFound(NotFoundKind::Email(e))) if e == email));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_password_by_email_method_works(pool: PgPool) {
        let email = user::Email::try_new("test@test.com").expect("Valid email from fixtures");

        let pg_users = PgUsers::new(pool);
        let result = pg_users.find_password_by_email(&email).await;
        assert_eq!(result.ok(), Some(String::from("test")));
    }

    #[sqlx::test(fixtures("./db/fixtures/seed.sql"))]
    async fn find_password_by_email_with_not_found(pool: PgPool) {
        use domain::user::{Error, NotFoundKind};
        let email = user::Email::try_new("jeff@jeff.com").expect("Valid email not in fixtures");

        let pg_users = PgUsers::new(pool);
        let result = pg_users.find_password_by_email(&email).await;
        assert!(matches!(result, Err(Error::NotFound(NotFoundKind::Email(e))) if e == email));
    }
}
