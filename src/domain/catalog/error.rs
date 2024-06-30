use thiserror::Error;

use super::{DescriptionError, Id, Name, NameError, ParseIdError, ProductsError};
use crate::product;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Conflict(ConflictKind),
    #[error(transparent)]
    Internal(Box<dyn std::error::Error>),
    #[error(transparent)]
    NotFound(NotFoundKind),
    #[error(transparent)]
    Validation(ValidationKind),
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

impl From<DescriptionError> for Error {
    fn from(value: DescriptionError) -> Self {
        Self::Validation(ValidationKind::Description(value))
    }
}

impl From<ParseIdError> for Error {
    fn from(value: ParseIdError) -> Self {
        Self::Validation(ValidationKind::Id(value))
    }
}

impl From<NameError> for Error {
    fn from(value: NameError) -> Self {
        Self::Validation(ValidationKind::Name(value))
    }
}

impl From<ProductsError> for Error {
    fn from(value: ProductsError) -> Self {
        Self::Validation(ValidationKind::Products(value))
    }
}

impl From<product::Error> for Error {
    fn from(value: product::Error) -> Self {
        match value {
            product::Error::Conflict(kind) => Self::Conflict(ConflictKind::Product(kind)),
            product::Error::Internal(src) => Self::Internal(src),
            product::Error::NotFound(kind) => Self::NotFound(NotFoundKind::Product(kind)),
            product::Error::Validation(kind) => Self::Validation(ValidationKind::Product(kind)),
        }
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

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ValidationKind {
    #[error(transparent)]
    Description(DescriptionError),
    #[error(transparent)]
    Id(ParseIdError),
    #[error(transparent)]
    Name(NameError),
    #[error(transparent)]
    Product(product::ValidationKind),
    #[error(transparent)]
    Products(ProductsError),
}
