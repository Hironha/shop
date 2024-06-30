use super::{Extras, Id, Name, Price};
use crate::catalog;
use crate::metadata;

#[derive(Clone, Debug)]
pub struct Product {
    pub(super) id: Id,
    pub(super) catalog_id: catalog::Id,
    pub(super) name: Name,
    pub(super) price: Price,
    pub(super) extras: Extras,
    pub(super) metadata: metadata::Metadata,
}

impl Product {
    #[must_use]
    pub fn new(catalog_id: catalog::Id, name: Name, price: Price, extras: Extras) -> Self {
        Self {
            id: Id::new(),
            catalog_id,
            name,
            price,
            extras,
            metadata: metadata::Metadata::new(),
        }
    }

    #[must_use]
    pub fn config(config: Config) -> Self {
        Self {
            id: config.id,
            catalog_id: config.catalog_id,
            name: config.name,
            price: config.price,
            extras: config.extras.unwrap_or_default(),
            metadata: config.metadata,
        }
    }

    #[must_use]
    pub fn id(&self) -> Id {
        self.id
    }

    #[must_use]
    pub fn catalog_id(&self) -> catalog::Id {
        self.catalog_id
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
    pub fn extras(&self) -> &Extras {
        &self.extras
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
    pub catalog_id: catalog::Id,
    pub name: Name,
    pub price: Price,
    pub extras: Option<Extras>,
    pub metadata: metadata::Metadata,
}

#[derive(Clone, Debug)]
pub struct Setter(Product);

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
    pub fn extras(mut self, extras: Extras) -> Self {
        self.0.extras = extras;
        self
    }

    #[must_use]
    pub fn commit(mut self) -> Product {
        self.0.metadata.update();
        self.0
    }
}
