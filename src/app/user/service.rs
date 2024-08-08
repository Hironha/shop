mod dto;
pub mod mail;
pub mod session;

use domain::user;
use domain::user::password::{Encrypter, Password};
pub use dto::{
    LoginError, LoginInput, LogoutError, LogoutInput, RegisterError, RegisterInput, VerifyEmail,
};

#[derive(Clone, Debug)]
pub struct UserService<T, R, S, U> {
    users: T,
    sessions: R,
    encrypter: S,
    mailer: U,
}

impl<T, R, S, U> UserService<T, R, S, U>
where
    T: user::Repository,
    R: session::Manager,
    S: Encrypter,
    U: mail::Mailer,
{
    pub fn new(users: T, sessions: R, encrypter: S, mailer: U) -> Self {
        Self {
            users,
            sessions,
            encrypter,
            mailer,
        }
    }
}

impl<T, R, S, U> UserService<T, R, S, U>
where
    T: user::Repository,
    R: session::Manager,
    S: Encrypter,
    U: mail::Mailer,
{
    pub async fn login(&mut self, input: LoginInput) -> Result<user::Id, LoginError> {
        let password = self.users.find_password_by_email(&input.email).await?;
        if !self.encrypter.verify(&password, &input.password) {
            return Err(LoginError::Credentials);
        }

        let user = self.users.find_by_email(&input.email).await?;
        if !user.is_email_verified() {
            return Err(LoginError::Unverified);
        }

        match self.sessions.create(&user).await {
            Ok(()) | Err(session::Error::AlreadyExists(_)) => Ok(user.id()),
            Err(err) => Err(LoginError::Session(err)),
        }
    }

    pub async fn logout(&mut self, input: LogoutInput) -> Result<(), LogoutError> {
        match self.sessions.revoke(input.user_id).await {
            Ok(()) => Ok(()),
            Err(session::Error::NotFound(id)) => Err(LogoutError::NotFound(id)),
            Err(err) => Err(LogoutError::Session(err)),
        }
    }

    pub async fn register(&mut self, input: RegisterInput) -> Result<(), RegisterError> {
        let user = user::User::new(input.username, input.email);
        let password = Password::new(&input.password, &self.encrypter);
        self.users.create(&user, &password).await?;

        let mail = mail::MailKind::Welcome(user.id());
        self.mailer.send(mail).await?;

        Ok(())
    }

    pub async fn verify_email(&mut self, input: VerifyEmail) -> Result<(), user::Error> {
        let user = self.users.find_by_email(&input.email).await?;
        let mail = mail::MailKind::Verification(user.id());
        if let Err(err) = self.mailer.send(mail).await {
            return Err(user::Error::any(err));
        }

        Ok(())
    }
}
