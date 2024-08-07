use std::fmt;

use thiserror::Error;
use uuid::Uuid;

use domain::user;

#[allow(clippy::unused_async)]
pub trait Manager {
    async fn create(&mut self, user: &user::User) -> Option<Id>;
    async fn refresh(&mut self, id: Id);
    async fn revoke(&mut self, id: Id);
    async fn validate(&self, id: Id) -> bool;
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn parse_str(value: &str) -> Result<Self, ParseIdError> {
        match Uuid::try_parse(value) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(_) => Err(ParseIdError(Box::from(value))),
        }
    }
}

impl Id {
    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<&str> for Id {
    type Error = ParseIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse_str(value)
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Provided string `{0}` is not a valid session id")]
pub struct ParseIdError(Box<str>);
