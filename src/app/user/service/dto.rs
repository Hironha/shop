use thiserror::Error;

use domain::user;

use super::mail;
use super::session;

#[derive(Clone, Debug)]
pub struct LoginInput {
    pub email: user::Email,
    pub password: String,
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Invalid user credentials")]
    Credentials,
    #[error(transparent)]
    Session(#[from] session::Error),
    #[error("User not verified. Verify your email before trying to login")]
    Unverified,
    #[error(transparent)]
    User(#[from] user::Error),
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

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error(transparent)]
    Mail(#[from] mail::SendMailError),
    #[error(transparent)]
    User(#[from] user::Error),
}

#[derive(Clone, Debug)]
pub struct VerifyEmail {
    pub email: user::Email,
}
