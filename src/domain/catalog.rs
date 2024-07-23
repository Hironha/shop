mod entity;
mod error;
mod repository;
mod variants;
mod vo;

pub use entity::{Catalog, Config};
pub use error::{ConflictKind, Error, NotFoundKind, ValidationKind};
pub use repository::{ListQuery, Pagination, Repository};
pub use variants::CatalogProducts;
pub use vo::{
    Description, DescriptionError, Id, Name, NameError, ParseIdError, Products, ProductsError,
};
