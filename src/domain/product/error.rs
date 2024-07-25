use thiserror::Error;

use super::{Id, Name};
use crate::catalog;
use crate::extra;

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
    pub fn catalog_not_found(catalog_id: catalog::Id) -> Self {
        Self::NotFound(NotFoundKind::CatalogId(catalog_id))
    }

    #[must_use]
    pub fn extra_not_found(extra_id: extra::Id) -> Self {
        Self::NotFound(NotFoundKind::ExtraId(extra_id))
    }

    #[must_use]
    pub fn id_conflict(id: Id) -> Self {
        Self::Conflict(ConflictKind::Id(id))
    }

    #[must_use]
    pub fn name_conflict(name: Name) -> Self {
        Self::Conflict(ConflictKind::Name(name))
    }

    #[must_use]
    pub fn id_not_found(id: Id, catalog_id: catalog::Id) -> Self {
        Self::NotFound(NotFoundKind::Id { id, catalog_id })
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ConflictKind {
    #[error("Product with id `{0}` already exists")]
    Id(Id),
    #[error("Product with name `{0}` already exists")]
    Name(Name),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum NotFoundKind {
    #[error("Product with id `{id}` not found for catalog {catalog_id}")]
    Id { id: Id, catalog_id: catalog::Id },
    #[error("Product catalog with id `{0}` not found")]
    CatalogId(catalog::Id),
    #[error("Product extra with id `{0}` not found")]
    ExtraId(extra::Id),
}
