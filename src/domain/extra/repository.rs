use super::error::Error;
use super::{Id, Extra};

// TODO: remove when stabilized
#[allow(async_fn_in_trait)]
pub trait Repository: Send + Clone {
    async fn all(&self) -> Result<Vec<Extra>, Error>;
    async fn create(&mut self, extra: &Extra) -> Result<(), Error>;
    async fn delete(&mut self, id: Id) -> Result<Extra, Error>;
    async fn find(&self, id: Id) -> Result<Extra, Error>;
    async fn find_many(&self, ids: &[Id]) -> Result<Vec<Extra>, Error>;
    async fn update(&mut self, extra: &Extra) -> Result<(), Error>;
}
