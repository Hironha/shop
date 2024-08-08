use thiserror::Error;

use domain::user;

pub trait Manager {
    async fn create(&mut self, user: &user::User) -> Result<(), Error>;
    async fn refresh(&mut self, id: user::Id) -> Result<(), Error>;
    async fn revoke(&mut self, id: user::Id) -> Result<(), Error>;
    async fn validate(&self, id: user::Id) -> Result<bool, Error>;
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Provided string `{0}` is not a valid session id")]
pub struct ParseIdError(Box<str>);

#[derive(Debug, Error)]
pub enum Error {
    #[error("Session for user with id `{0}` already exists")]
    AlreadyExists(user::Id),
    #[error("Session for user with id `{0}` not found")]
    NotFound(user::Id),
    #[error(transparent)]
    Internal(Box<dyn std::error::Error>),
}

impl Error {
    pub fn any(err: impl Into<Box<dyn std::error::Error>>) -> Self {
        Self::Internal(err.into())
    }
}
