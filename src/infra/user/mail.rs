use crate::app::user::service::mail::{MailKind, Mailer, SendMailError};

#[derive(Clone, Debug)]
pub struct LettreMailer;

impl Mailer for LettreMailer {
    async fn send(&mut self, _kind: MailKind) -> Result<(), SendMailError> {
        println!("Mailer service not yet implemented");
        Ok(())
    }
}
