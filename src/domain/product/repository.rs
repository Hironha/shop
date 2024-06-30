use super::error::Error;
use super::{Id, Product};
use crate::catalog;

// TODO: remove when stabilized
#[allow(async_fn_in_trait)]
pub trait Repository: Send + Clone {
    async fn create(&mut self, product: &Product) -> Result<(), Error>;
    async fn delete(&mut self, id: Id, catalog_id: catalog::Id) -> Result<Product, Error>;
    async fn find(&self, id: Id, catalog_id: catalog::Id) -> Result<Product, Error>;
    async fn update(&mut self, product: &Product) -> Result<(), Error>;
}
