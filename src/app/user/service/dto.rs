use thiserror::Error;

use domain::user;

use super::session;

#[derive(Clone, Debug)]
pub struct LoginInput {
    pub email: user::Email,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct LogoutInput {
    pub user_id: user::Id,
}

#[derive(Debug, Error)]
pub enum LogoutError {
    #[error("Session for user with id `{0}` not found")]
    NotFound(user::Id),
    #[error(transparent)]
    Session(session::Error),
}

#[derive(Clone, Debug)]
pub struct RegisterInput {
    pub username: user::Username,
    pub email: user::Email,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct VerifyEmail {
    pub email: user::Email,
}
