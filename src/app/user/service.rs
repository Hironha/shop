mod dto;
pub mod session;

use domain::user;
use domain::user::password::{Encrypter, Password};
pub use dto::{LoginInput, RegisterInput};

#[derive(Clone, Debug)]
pub struct UserService<T, R, S> {
    encrypter: T,
    users: R,
    sessions: S,
}

impl<T, R, S> UserService<T, R, S>
where
    T: Encrypter,
    R: user::Repository,
    S: session::Manager,
{
    pub fn new(encrypter: T, users: R, sessions: S) -> Self {
        Self {
            encrypter,
            users,
            sessions,
        }
    }
}

impl<T, R, S> UserService<T, R, S>
where
    T: Encrypter,
    R: user::Repository,
    S: session::Manager,
{
    pub async fn login(&mut self, input: LoginInput) -> Result<session::Id, user::Error> {
        let password = self.users.find_password(&input.email).await?;
        if !self.encrypter.verify(&password, &input.password) {
            return Err(user::Error::Credentials);
        }

        let user = self.users.find_by_email(&input.email).await?;
        if !user.is_email_verified() {
            return Err(user::Error::any("Verify your email before you can login"));
        }

        let Some(session_id) = self.sessions.create(&user).await else {
            return Err(user::Error::any("Failed creating user session"));
        };

        Ok(session_id)
    }

    #[allow(clippy::unused_async)]
    pub async fn logout(&mut self) -> Result<(), user::Error> {
        // self.session.revoke(input.session_id).await?;
        todo!()
    }

    pub async fn register(&mut self, input: RegisterInput) -> Result<(), user::Error> {
        let user = user::User::new(input.username, input.email);
        let password = Password::new(&input.password, &self.encrypter);
        self.users.create(&user, &password).await
    }

    #[allow(clippy::unused_async)]
    pub async fn verify_email(&mut self) -> Result<(), user::Error> {
        todo!()
    }
}
