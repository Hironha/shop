mod dto;

pub use dto::{LoginInput, RegisterInput};

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
    pub async fn login(&mut self, input: LoginInput) -> Result<String, user::Error> {
        let user_password = self.users.find_password(input.email).await?;
        let password = self.encrypter.encrypt(&input.password);
        if !self.encrypter.verify(&password, &user_password) {
            return Err(user::Error::Credentials);
        }

        Ok(String::from("token"))
    }

    pub async fn register(&mut self, input: RegisterInput) -> Result<(), user::Error> {
        let user = user::User::new(input.username, input.email);
        let password = self.encrypter.encrypt(&input.password);
        self.users.create(user, password).await
    }
}
