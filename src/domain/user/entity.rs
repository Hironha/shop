use super::{Email, Id, Username};
use crate::core::metadata::Metadata;

#[derive(Clone, Debug)]
pub struct User {
    pub(super) id: Id,
    pub username: Username,
    pub email: Email,
    email_verified: bool,
    pub metadata: Metadata,
}

impl User {
    #[must_use]
    pub fn new(username: Username, email: Email) -> Self {
        Self {
            id: Id::new(),
            username,
            email,
            email_verified: false,
            metadata: Metadata::new(),
        }
    }
}

impl User {
    #[must_use]
    pub fn id(&self) -> Id {
        self.id
    }

    #[must_use]
    pub fn is_email_verified(&self) -> bool {
        self.email_verified
    }
}
