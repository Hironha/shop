use std::num::{NonZeroU32, NonZeroU8};

use super::{Catalog, ProductCatalog, Error, Id};

#[allow(async_fn_in_trait)]
pub trait Repository: Send + Clone {
    async fn create(&mut self, catalog: &Catalog) -> Result<(), Error>;
    async fn delete(&self, id: Id) -> Result<ProductCatalog, Error>;
    async fn find(&self, id: Id) -> Result<ProductCatalog, Error>;
    async fn list(&self, query: ListQuery) -> Result<Pagination, Error>;
    async fn update(&mut self, catalog: &Catalog) -> Result<(), Error>;
}

#[derive(Clone, Debug)]
pub struct ListQuery {
    pub page: NonZeroU32,
    pub limit: NonZeroU8,
}

#[derive(Clone, Debug)]
pub struct Pagination {
    pub count: u64,
    pub page: NonZeroU32,
    pub limit: NonZeroU8,
    pub items: Vec<ProductCatalog>,
}
