use thiserror::Error;

use super::{Id, Name};
use crate::product;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Conflict(ConflictKind),
    #[error(transparent)]
    Internal(Box<dyn std::error::Error>),
    #[error(transparent)]
    NotFound(NotFoundKind),
}

impl Error {
    /// Utility function to create [`Error::Internal`] without manually
    /// boxing the error
    #[must_use]
    pub fn any(err: impl Into<Box<dyn std::error::Error>>) -> Self {
        Self::Internal(err.into())
    }

    #[must_use]
    pub fn id_conflict(id: Id) -> Self {
        Self::Conflict(ConflictKind::Id(id))
    }

    #[must_use]
    pub fn id_not_found(id: Id) -> Self {
        Self::NotFound(NotFoundKind::Id(id))
    }

    #[must_use]
    pub fn name_conflict(name: Name) -> Self {
        Self::Conflict(ConflictKind::Name(name))
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ConflictKind {
    #[error("Product catalog with id `{0}` already exists")]
    Id(Id),
    #[error("Product catalog wit name `{0}` already exists")]
    Name(Name),
    #[error(transparent)]
    Product(product::ConflictKind),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum NotFoundKind {
    #[error("Product catalog with id {0} not found")]
    Id(Id),
    #[error(transparent)]
    Product(product::NotFoundKind),
}
