mod dto;
pub mod mail;
pub mod session;

use domain::user;
use domain::user::password::{Encrypter, Password};
pub use dto::{LoginInput, LogoutError, LogoutInput, RegisterInput, VerifyEmail};

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
    pub async fn login(&mut self, input: LoginInput) -> Result<user::Id, user::Error> {
        let password = self.users.find_password_by_email(&input.email).await?;
        if !self.encrypter.verify(&password, &input.password) {
            return Err(user::Error::Credentials);
        }

        let user = self.users.find_by_email(&input.email).await?;
        if !user.is_email_verified() {
            return Err(user::Error::any("Verify your email before you can login"));
        }

        match self.sessions.create(&user).await {
            Ok(()) | Err(session::Error::AlreadyExists(_)) => Ok(user.id()),
            Err(err) => Err(user::Error::any(err)),
        }
    }

    pub async fn logout(&mut self, input: LogoutInput) -> Result<(), LogoutError> {
        match self.sessions.revoke(input.user_id).await {
            Ok(()) => Ok(()),
            Err(session::Error::NotFound(id)) => Err(LogoutError::NotFound(id)),
            Err(err) => Err(LogoutError::Session(err)),
        }
    }

    pub async fn register(&mut self, input: RegisterInput) -> Result<(), user::Error> {
        let user = user::User::new(input.username, input.email);
        let password = Password::new(&input.password, &self.encrypter);
        self.users.create(&user, &password).await?;

        if let Err(err) = self.mailer.send(mail::MailKind::Welcome).await {
            return Err(user::Error::any(err));
        }

        Ok(())
    }

    pub async fn verify_email(&mut self, input: VerifyEmail) -> Result<(), user::Error> {
        let _user = self.users.find_by_email(&input.email).await?;

        if let Err(err) = self.mailer.send(mail::MailKind::Verification).await {
            return Err(user::Error::any(err));
        }

        Ok(())
    }
}
