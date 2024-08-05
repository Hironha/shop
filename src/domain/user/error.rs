use thiserror::Error;

use super::{Email, Id};

#[derive(Debug, Error)]
#[must_use]
pub enum Error {
    #[error(transparent)]
    Conflict(#[from] ConflictKind),
    #[error(transparent)]
    Internal(Box<dyn std::error::Error>),
}

impl Error {
    pub fn any(err: impl Into<Box<dyn std::error::Error>>) -> Self {
        Self::Internal(err.into())
    }

    pub fn email_conflict(email: Email) -> Self {
        Self::Conflict(ConflictKind::Email(email))
    }

    pub fn id_conflict(id: Id) -> Self {
        Self::Conflict(ConflictKind::Id(id))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum ConflictKind {
    #[error("User with id `{0}` already exists")]
    Id(Id),
    #[error("User with email `{0}` already exists")]
    Email(Email),
}
