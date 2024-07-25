use thiserror::Error;

use super::{Id, IdError, Name, NameError};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Conflict(ConflictKind),
    #[error("Product extra with id `{0}` not found")]
    NotFound(Id),
    #[error(transparent)]
    Internal(Box<dyn std::error::Error>),
}

impl Error {
    #[must_use]
    pub fn any(err: impl Into<Box<dyn std::error::Error>>) -> Self {
        Self::Internal(err.into())
    }

    #[must_use]
    pub fn id_conflict(id: Id) -> Self {
        Self::Conflict(ConflictKind::Id(id))
    }

    #[must_use]
    pub fn name_conflict(name: Name) -> Self {
        Self::Conflict(ConflictKind::Name(name))
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ConflictKind {
    #[error("Product extra with id `{0}` already exists")]
    Id(Id),
    #[error("Product extra with name `{0}` already exists")]
    Name(Name),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ValidationKind {
    #[error(transparent)]
    Id(IdError),
    #[error(transparent)]
    Name(NameError),
}
