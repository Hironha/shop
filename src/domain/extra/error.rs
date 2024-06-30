use thiserror::Error;

use super::{Id, IdError, Name, NameError};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Validation(ValidationKind),
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
    pub fn id_err(err: IdError) -> Self {
        Self::Validation(ValidationKind::Id(err))
    }

    #[must_use]
    pub fn name_conflict(name: Name) -> Self {
        Self::Conflict(ConflictKind::Name(name))
    }

    #[must_use]
    pub fn name_err(err: NameError) -> Self {
        Self::Validation(ValidationKind::Name(err))
    }
}

impl From<NameError> for Error {
    fn from(value: NameError) -> Self {
        Self::name_err(value)
    }
}

impl From<IdError> for Error {
    fn from(value: IdError) -> Self {
        Self::id_err(value)
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
