use std::fmt;

use thiserror::Error;
use uuid::Uuid;

use crate::product;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Id(Uuid);

impl Id {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Try parsing `value` into [`Id`]
    ///
    /// # Errors
    ///
    /// Returns an [`Err`] if `value` cannot be parsed into [`Id`]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Name(String);

impl Name {
    pub const MAX_LEN: usize = 64;

    /// Try parsing `name` into [`Name`]
    ///
    /// # Errors
    ///
    /// Returns an [`Err`] if `name` is not a valid [`Name`]
    pub fn new(name: impl Into<String>) -> Result<Self, NameError> {
        let mut name: String = name.into();
        name.drain(..name.len() - name.trim_start().len());
        name.drain(name.trim_end().len()..);

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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Description(String);

impl Description {
    pub const MAX_LEN: usize = 128;

    /// Try parsing `description` into [`Description`]
    ///
    /// # Errors
    ///
    /// Returns an [`Err`] if `description` is not a valid [`Description`]
    pub fn new(description: impl Into<String>) -> Result<Self, DescriptionError> {
        let mut description: String = description.into();
        description.drain(..description.len() - description.trim_start().len());
        description.drain(description.trim_end().len()..);

        if description.len() > Self::MAX_LEN {
            return Err(DescriptionError::Length);
        }

        Ok(Self(description))
    }
}

impl Description {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Products(Vec<product::Product>);

impl Products {
    pub const MAX_LEN: u8 = 64;

    /// Try parsing `products` into [`Products`]
    ///
    /// # Errors
    ///
    /// Returns an [`Err`] if `products` have more items than allowed
    pub fn new(products: Vec<product::Product>) -> Result<Self, ProductsError> {
        if products.len() > usize::from(Self::MAX_LEN) {
            return Err(ProductsError::Length);
        }

        Ok(Self(products))
    }
}

impl Products {
    #[must_use]
    pub fn as_slice(&self) -> &[product::Product] {
        &self.0
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &product::Product> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Provided string `{0}` is not a valid product catalog id`")]
pub struct ParseIdError(pub Box<str>);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum NameError {
    #[error("Product catalog name cannot have more than {len} characters", len = Name::MAX_LEN)]
    Length,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum DescriptionError {
    #[error("Product catalog description cannot have more than {len} characters", len = Description::MAX_LEN)]
    Length,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ProductsError {
    #[error("Product catalog cannot have more than {len} products", len = Products::MAX_LEN)]
    Length,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_id_works() {
        let valid_id_str = Id::new().to_string();
        assert!(Id::parse_str(&valid_id_str).is_ok());

        let invalid_id_str = "Test";
        assert_eq!(
            Id::parse_str(invalid_id_str),
            Err(ParseIdError(Box::from(invalid_id_str)))
        );
    }

    #[test]
    fn new_name_works() {
        let simple = ["Test", "Hamburgers", "Sushi", "Combos"];
        let compound = ["Chees Burgers", "Promotional Products"];
        let untrimmed = [" Test ", " Hamburgers", "Sushi ", " Cheese Burgers "];
        for n in simple.into_iter().chain(compound).chain(untrimmed) {
            let name = Name::new(n);
            assert!(name.is_ok());
            assert_eq!(name.unwrap().as_str(), n.trim());
        }

        let big = ["a"; Name::MAX_LEN + 1].join("");
        assert_eq!(Name::new(big), Err(NameError::Length));
    }

    #[test]
    fn new_description_works() {
        let simple = ["Test", "Description"];
        let composed = ["All products 100% vegan!", "Best burgers in the country"];
        let untrimmed = [" Test ", "Description ", " Best burgers"];
        for d in simple.into_iter().chain(composed).chain(untrimmed) {
            let description = Description::new(d);
            assert!(description.is_ok());
            assert_eq!(description.unwrap().as_str(), d.trim());
        }

        let big = ["a"; Description::MAX_LEN + 1].join("");
        assert_eq!(Description::new(big), Err(DescriptionError::Length));
    }
}
