use std::fmt;

use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

use crate::extra;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    /// Try parsing `value` into [`Id`]
    ///
    /// # Errors
    ///
    /// Returns an [`Err`] if `value` is not a valid [`Id`]
    pub fn parse_str(value: &str) -> Result<Self, ParseIdError> {
        match Uuid::parse_str(value) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(_) => Err(ParseIdError(Box::from(value))),
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
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Name(String);

impl Name {
    const MAX_LEN: usize = 64;

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

#[derive(Clone, Debug, Default)]
pub struct Extras(Vec<extra::Extra>);

impl Extras {
    pub const MAX_LEN: usize = 32;

    /// Try parsing `extras` into [`Extras`]
    ///
    /// # Errors
    ///
    /// Returns an [`Err`] if `extras` has more items than allowed
    pub fn new(extras: Vec<extra::Extra>) -> Result<Self, ExtrasError> {
        if extras.len() > Self::MAX_LEN {
            return Err(ExtrasError::Length);
        }

        Ok(Self(extras))
    }

    #[must_use]
    pub fn as_slice(&self) -> &[extra::Extra] {
        &self.0
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Provided string `{0} is not a valid product id`")]
pub struct ParseIdError(pub Box<str>);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum NameError {
    #[error("Product name cannot have more than {len} characteres", len = Name::MAX_LEN)]
    Length,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ExtrasError {
    #[error("Product cannot have more than {len} extras", len = Extras::MAX_LEN)]
    Length,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_name() {
        let single = vec!["Hironha", "Carlos", "John", "Elon", "Marx"];
        let composed = vec!["João Vitor", "José Bonifácio"];

        for name in single.into_iter().chain(composed.into_iter()) {
            assert!(Name::new(name).is_ok());
        }

        let big_name = vec!['a'; Name::MAX_LEN + 1].into_iter().collect::<String>();
        assert_eq!(Name::new(big_name), Err(NameError::Length));
    }

    #[test]
    fn parse_id() {
        let invalid = "invalid-id";
        assert_eq!(
            Id::parse_str(invalid),
            Err(ParseIdError(Box::from(invalid)))
        );

        let valid = Uuid::now_v7().to_string();
        assert!(Id::parse_str(&valid).is_ok());
    }

    #[test]
    fn parse_extras() {
        let small_extras: Vec<extra::Extra> = vec![];
        assert!(Extras::new(small_extras).is_ok());

        let name = extra::Name::new("Test").unwrap();
        let extra = extra::Extra::new(name, extra::Price::from_cents(20));
        let big_extras: Vec<extra::Extra> = vec![extra; Extras::MAX_LEN + 1];
        assert_eq!(Extras::new(big_extras).err(), Some(ExtrasError::Length));
    }
}
