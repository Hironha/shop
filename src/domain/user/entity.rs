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

    #[must_use]
    pub fn config(cfg: UserConfig) -> Self {
        Self {
            id: cfg.id,
            username: cfg.username,
            email: cfg.email,
            email_verified: cfg.email_verified,
            metadata: cfg.metadata,
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

#[derive(Clone, Debug)]
pub struct UserConfig {
    pub id: Id,
    pub username: Username,
    pub email: Email,
    pub email_verified: bool,
    pub metadata: Metadata,
}
