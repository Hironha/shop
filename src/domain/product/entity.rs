use super::{Extras, Id, Name, Price};
use crate::catalog;
use crate::metadata;

#[derive(Clone, Debug)]
pub struct Product {
    pub(super) id: Id,
    pub(super) catalog_id: catalog::Id,
    pub name: Name,
    pub price: Price,
    pub extras: Extras,
    pub metadata: metadata::Metadata,
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
}

impl Product {
    #[must_use]
    pub fn id(&self) -> Id {
        self.id
    }

    #[must_use]
    pub fn catalog_id(&self) -> catalog::Id {
        self.catalog_id
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
