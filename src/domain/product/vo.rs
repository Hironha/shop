use std::fmt;

use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

use crate::core::string::trim_in_place;
use crate::extra;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Id(Uuid);

impl Id {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn parse_str(value: &str) -> Result<Self, ParseIdError> {
        match Uuid::parse_str(value) {
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

    #[must_use]
    pub fn take(self) -> String {
        self.0
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

    pub fn new(extras: Vec<extra::Extra>) -> Result<Self, ExtrasError> {
        if extras.len() > Self::MAX_LEN {
            return Err(ExtrasError::Length);
        }

        Ok(Self(extras))
    }
}

impl Extras {
    #[must_use]
    pub fn as_slice(&self) -> &[extra::Extra] {
        &self.0
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &extra::Extra> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn take(self) -> Vec<extra::Extra> {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    Brazillian,
    Burger,
    French,
    IceCream,
    Italian,
    Japanese,
    Korean,
    Libanese,
    Vegan,
}

impl Kind {
    pub fn parse_str(value: &str) -> Result<Self, ParseKindError> {
        match value {
            "brazillian" => Ok(Self::Brazillian),
            "burger" => Ok(Self::Burger),
            "french" => Ok(Self::French),
            "ice_cream" => Ok(Self::IceCream),
            "italian" => Ok(Self::Italian),
            "japanese" => Ok(Self::Japanese),
            "korean" => Ok(Self::Korean),
            "libanese" => Ok(Self::Libanese),
            "vegan" => Ok(Self::Vegan),
            other => Err(ParseKindError(Box::from(other))),
        }
    }
}

impl Kind {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Brazillian => "brazillian",
            Self::Burger => "burger",
            Self::French => "french",
            Self::IceCream => "ice_cream",
            Self::Italian => "italian",
            Self::Japanese => "japanese",
            Self::Korean => "korean",
            Self::Libanese => "libanese",
            Self::Vegan => "vegan",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl TryFrom<&str> for Kind {
    type Error = ParseKindError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse_str(value)
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Provided string `{0} is not a valid product id`")]
pub struct ParseIdError(pub Box<str>);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Provided string `{0}` is not a valid kind of product")]
pub struct ParseKindError(pub Box<str>);

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

    #[test]
    fn kind_back_and_forth_str() {
        let kinds = [
            Kind::Brazillian,
            Kind::Burger,
            Kind::French,
            Kind::IceCream,
            Kind::Italian,
            Kind::Japanese,
            Kind::Korean,
            Kind::Libanese,
            Kind::Vegan,
        ];

        for kind in kinds {
            let parsed = Kind::parse_str(kind.as_str());
            assert_eq!(parsed, Ok(kind));
            assert_eq!(parsed.unwrap().as_str(), kind.as_str());
        }
    }
}
