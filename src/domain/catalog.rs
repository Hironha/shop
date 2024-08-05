mod entity;
mod error;
mod repository;
mod variants;
mod vo;

pub use entity::{Catalog, CatalogConfig};
pub use error::{ConflictKind, Error, NotFoundKind};
pub use repository::{ListQuery, Pagination, Repository};
pub use variants::ProductCatalog;
pub use vo::{
    Description, DescriptionError, Id, Name, NameError, ParseIdError, Products, ProductsError,
};
