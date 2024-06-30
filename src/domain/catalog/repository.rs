use super::{Catalog, Error, Id};

#[allow(async_fn_in_trait)]
pub trait Repository: Send + Clone {
    async fn create(&mut self, catalog: &Catalog) -> Result<(), Error>;
    async fn delete(&self, id: Id) -> Result<Catalog, Error>;
    async fn find(&self, id: Id) -> Result<Catalog, Error>;
    async fn list(&self, query: ListQuery) -> Result<Pagination, Error>;
    async fn update(&mut self, catalog: &Catalog) -> Result<(), Error>;
}

#[derive(Clone, Debug)]
pub struct ListQuery {
    pub page: u64,
    pub limit: u64,
}

#[derive(Clone, Debug)]
pub struct Pagination {
    pub count: u64,
    pub page: u64,
    pub limit: u64,
    pub items: Vec<Catalog>,
}
