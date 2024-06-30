use std::fmt;

use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Id(Uuid);

impl Id {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    #[must_use]
    pub fn uuid(&self) -> Uuid {
        self.0
    }

    /// Try parsing a `value` into [`Id`]
    ///
    /// # Errors
    ///
    /// Returns an [`Err`] if `value` is not a valid [`Id`]
    pub fn parse_str(value: &str) -> Result<Self, IdError> {
        match Uuid::parse_str(value) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(_) => Err(IdError::Parse(Box::from(value))),
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for Id {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Name(String);

impl Name {
    pub const MAX_LEN: usize = 128;

    /// Try parsing `name` into [`Name`]
    ///
    /// # Errors
    ///
    /// Returns an [`Err`] if `name` does not fit into [`Name`] constraints
    pub fn new(name: impl Into<String>) -> Result<Self, NameError> {
        let name: String = name.into();
        if name.len() > Self::MAX_LEN {
            return Err(NameError::Length);
        }

        Ok(Self(name))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Price(Decimal);

impl Price {
    #[must_use]
    pub fn from_cents(value: u64) -> Self {
        let num = i64::try_from(value).unwrap_or(i64::MAX);
        Self(Decimal::new(num, 2))
    }

    #[must_use]
    pub fn from_decimal(value: Decimal) -> Self {
        Self(value.trunc_with_scale(2))
    }

    #[must_use]
    pub fn decimal(&self) -> Decimal {
        self.0
    }
}

impl From<Decimal> for Price {
    fn from(value: Decimal) -> Self {
        Self::from_decimal(value)
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum IdError {
    #[error("Provided string `{0}` is not a valid product extra id")]
    Parse(Box<str>),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum NameError {
    #[error("Product extra name cannot have more than {len} characters", len = Name::MAX_LEN)]
    Length,
}
