use thiserror::Error;

pub trait Mailer {
    async fn send(&mut self, kind: MailKind) -> Result<(), SendMailError>;
}

#[derive(Clone, Debug)]
pub enum MailKind {
    Welcome,
    Verification,
}

#[derive(Debug, Error)]
pub enum SendMailError {
    #[error(transparent)]
    Internal(Box<dyn std::error::Error>),
}
