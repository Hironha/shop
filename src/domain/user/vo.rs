use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::core::string::trim_in_place;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(try_from = "&str")]
pub struct Id(Uuid);

impl Id {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    #[must_use]
    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    pub fn parse_str(value: &str) -> Result<Self, ParseIdError> {
        match Uuid::try_parse(value) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(_) => Err(ParseIdError(Box::from(value))),
        }
    }
}

impl Id {
    #[must_use]
    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self::from_uuid(value)
    }
}

impl TryFrom<&str> for Id {
    type Error = ParseIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse_str(value)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Username(String);

impl Username {
    pub const MAX_LEN: usize = 64;

    pub fn try_new(username: impl Into<String>) -> Result<Self, UsernameError> {
        let mut username: String = username.into();
        trim_in_place(&mut username);

        if username.len() > Self::MAX_LEN {
            return Err(UsernameError::Length);
        }

        Ok(Self(username))
    }
}

impl Username {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Username {
    type Error = UsernameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Email(String);

impl Email {
    pub const MAX_LEN: usize = 256;

    pub fn try_new(email: impl Into<String>) -> Result<Self, EmailError> {
        let mut email: String = email.into();
        trim_in_place(&mut email);

        if email.len() > Self::MAX_LEN {
            return Err(EmailError::Length);
        }

        if !email.contains('@') {
            return Err(EmailError::Invalid(email.into()));
        }

        Ok(Self(email))
    }
}

impl Email {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Provided string `{0}` is not a valid user id")]
pub struct ParseIdError(Box<str>);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum UsernameError {
    #[error("Username cannot contain more than {max} characters", max = Username::MAX_LEN)]
    Length,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum EmailError {
    #[error("Email cannot contain more than {max} characters", max = Email::MAX_LEN)]
    Length,
    #[error("Provided email `{0}` is not valid")]
    Invalid(Box<str>),
}
