use super::password::Password;
use super::{Error, Id, User};

#[allow(async_fn_in_trait)]
pub trait Repository: Send + Clone {
    async fn create(&mut self, user: User, password: Password) -> Result<(), Error>;
    async fn find_password(&mut self, user_id: Id) -> Result<String, Error>;
}
