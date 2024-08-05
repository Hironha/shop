use domain::user;
use domain::user::password::{Encrypt, PasswordEncrypter};

#[derive(Clone, Debug)]
pub struct UserService<T, R> {
    encrypter: PasswordEncrypter<T>,
    users: R,
}

impl<T: Encrypt, R: user::Repository> UserService<T, R> {
    pub fn new(encrypter: T, users: R) -> Self {
        Self {
            encrypter: PasswordEncrypter::new(encrypter),
            users,
        }
    }
}

impl<T: Encrypt, R: user::Repository> UserService<T, R> {
    #[allow(clippy::unused_async)]
    pub async fn register(&mut self) -> Result<(), user::Error> {
        todo!()
    }
}
