use super::{Id, Name, Price};
use crate::metadata;

#[derive(Clone, Debug)]
pub struct Extra {
    pub(super) id: Id,
    pub name: Name,
    pub price: Price,
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
    pub fn metadata(&self) -> &metadata::Metadata {
        &self.metadata
    }

    pub fn set_updated(&mut self) {
        self.metadata.update();
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub id: Id,
    pub name: Name,
    pub price: Price,
    pub metadata: metadata::Metadata,
}
