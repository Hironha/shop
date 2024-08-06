use super::password::Password;
use super::{Email, Error, User};

#[allow(async_fn_in_trait)]
pub trait Repository: Send + Clone {
    async fn create(&mut self, user: &User, password: &Password) -> Result<(), Error>;
    async fn find_by_email(&self, email: &Email) -> Result<User, Error>;
    async fn find_password(&self, email: &Email) -> Result<String, Error>;
}
