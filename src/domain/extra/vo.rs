use std::fmt;

use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

use crate::core::string::trim_in_place;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Id(Uuid);

impl Id {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn parse_str(value: &str) -> Result<Self, IdError> {
        match Uuid::parse_str(value) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(_) => Err(IdError::Parse(Box::from(value))),
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

    pub fn new(name: impl Into<String>) -> Result<Self, NameError> {
        let mut name: String = name.into();
        trim_in_place(&mut name);

        if name.len() > Self::MAX_LEN {
            return Err(NameError::Length);
        }

        Ok(Self(name))
    }
}

impl Name {
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
    pub fn new(value: Decimal) -> Self {
        Self(value.trunc_with_scale(2))
    }

    #[must_use]
    pub fn from_cents(value: u64) -> Self {
        let num = i64::try_from(value).unwrap_or(i64::MAX);
        Self(Decimal::new(num, 2))
    }
}

impl Price {
    #[must_use]
    pub fn decimal(&self) -> Decimal {
        self.0
    }

    // TODO: maybe i should return an error here and improve performance
    #[must_use]
    pub fn to_cents(&self) -> u64 {
        use rust_decimal::prelude::ToPrimitive;
        let cents = self.0.saturating_mul(Decimal::new(100, 0));
        cents.to_u64().unwrap_or_default()
    }
}

impl From<Decimal> for Price {
    fn from(value: Decimal) -> Self {
        Self::new(value)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_name_works() {
        let simple = ["Cheddar", "Salad", "Sugar"];
        let composed = ["Big Cheddar", "Carrot Salad", "Brown Sugar"];
        for n in simple.into_iter().chain(composed) {
            assert!(Name::new(n).is_ok());
        }
    }

    #[test]
    fn new_name_with_whitespaces() {
        let at_start = [" Cheddar", "  Salad", "   Sugar"];
        let at_end = ["Cheddar ", "Salad  ", "Sugar    "];
        let both = [" Cheddar ", "  Feijão  ", "   Pó de guaraná   "];
        for n in at_start.into_iter().chain(at_end).chain(both) {
            let name = Name::new(n);
            assert_eq!(name.as_ref().map(Name::as_str), Ok(n.trim()));
        }
    }
}
