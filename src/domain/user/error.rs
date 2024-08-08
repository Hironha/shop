use thiserror::Error;

use super::{Email, Id};

#[derive(Debug, Error)]
#[must_use]
pub enum Error {
    #[error("User with email `{0}` already exists")]
    EmailConflict(Email),
    #[error("User with email `{0}` not found")]
    EmailNotFound(Email),
    #[error("User with id `{0}` already exists")]
    IdConflict(Id),
    #[error("User with id `{0}` not found")]
    IdNotFound(Id),
    #[error(transparent)]
    Internal(Box<dyn std::error::Error>),
}

impl Error {
    pub fn any(err: impl Into<Box<dyn std::error::Error>>) -> Self {
        Self::Internal(err.into())
    }
}
