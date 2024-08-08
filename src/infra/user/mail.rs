use crate::app::user::service::mail::{MailKind, Mailer, SendMailError};

#[derive(Clone, Debug)]
pub struct LettreMailer;

impl Mailer for LettreMailer {
    async fn send(&mut self, kind: MailKind) -> Result<(), SendMailError> {
        match kind {
            MailKind::Welcome(id) => println!("Welcome, `{id}`"),
            MailKind::Verification(id) => println!("Send new verification email for `{id}`"),
        };

        Ok(())
    }
}
