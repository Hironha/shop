use thiserror::Error;

use domain::user;

pub trait Mailer {
    async fn send(&mut self, kind: MailKind) -> Result<(), SendMailError>;
}

#[derive(Clone, Debug)]
pub enum MailKind {
    Verification(user::Id),
    Welcome(user::Id),
}

#[derive(Debug, Error)]
pub enum SendMailError {
    #[error(transparent)]
    Internal(Box<dyn std::error::Error>),
}
