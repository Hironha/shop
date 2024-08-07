use sqlx::types::time::OffsetDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::FromRow;

use domain::core::metadata::Metadata;
use domain::user;

#[derive(Clone, Debug, FromRow)]
pub struct UserModel {
    pub id: Uuid,
    #[sqlx(try_from = "String")]
    pub username: user::Username,
    #[sqlx(try_from = "String")]
    pub email: user::Email,
    pub email_verified: bool,
    #[sqlx(rename = "password_hash")]
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl UserModel {
    pub fn try_into_entity(self) -> Result<user::User, Box<dyn std::error::Error>> {
        let id = user::Id::from(self.id);
        let metadata = Metadata::configured(self.created_at, self.updated_at)?;
        let user = user::User::config(user::UserConfig {
            id,
            username: self.username,
            email: self.email,
            email_verified: self.email_verified,
            metadata,
        });

        Ok(user)
    }
}
