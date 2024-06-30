use super::{Id, Name, Price};
use crate::metadata;

#[derive(Clone, Debug)]
pub struct Extra {
    pub(super) id: Id,
    pub(super) name: Name,
    pub(super) price: Price,
    pub(super) metadata: metadata::Metadata,
}

impl Extra {
    #[must_use]
    pub fn new(name: Name, price: Price) -> Self {
        Self {
            id: Id::new(),
            name,
            price,
            metadata: metadata::Metadata::new(),
        }
    }

    #[must_use]
    pub fn config(config: Config) -> Self {
        Self {
            id: config.id,
            name: config.name,
            price: config.price,
            metadata: config.metadata,
        }
    }

    #[must_use]
    pub fn id(&self) -> Id {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &Name {
        &self.name
    }

    #[must_use]
    pub fn price(&self) -> Price {
        self.price
    }

    #[must_use]
    pub fn metadata(&self) -> &metadata::Metadata {
        &self.metadata
    }

    #[must_use]
    pub fn into_setter(self) -> Setter {
        Setter(self)
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub id: Id,
    pub name: Name,
    pub price: Price,
    pub metadata: metadata::Metadata,
}

#[derive(Debug)]
pub struct Setter(Extra);

impl Setter {
    #[must_use]
    pub fn name(mut self, name: Name) -> Self {
        self.0.name = name;
        self
    }

    #[must_use]
    pub fn price(mut self, price: Price) -> Self {
        self.0.price = price;
        self
    }

    #[must_use]
    pub fn commit(mut self) -> Extra {
        self.0.metadata.update();
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_product_extra() {
        let product_extra = Extra::new(Name::new("Test").unwrap(), Price::from_cents(2000));
        let prev_created_at = product_extra.metadata.created_at();
        let prev_updated_at = product_extra.metadata.updated_at();

        let new_name = "Updated";
        let new_price = Price::from_cents(3000);
        let updated = product_extra
            .into_setter()
            .name(Name::new(new_name).unwrap())
            .price(new_price)
            .commit();

        assert!(prev_created_at == updated.metadata.created_at());
        assert!(prev_updated_at < updated.metadata.updated_at());
        assert_eq!(updated.name().as_str(), new_name);
        assert_eq!(updated.price(), new_price);
    }
}
