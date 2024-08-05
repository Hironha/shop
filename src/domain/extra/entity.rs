use super::{Id, Name, Price};
use crate::core::metadata;

#[derive(Clone, Debug)]
pub struct Extra {
    pub(super) id: Id,
    pub name: Name,
    pub price: Price,
    pub metadata: metadata::Metadata,
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
    pub fn config(config: ExtraConfig) -> Self {
        Self {
            id: config.id,
            name: config.name,
            price: config.price,
            metadata: config.metadata,
        }
    }
}

impl Extra {
    #[must_use]
    pub fn id(&self) -> Id {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct ExtraConfig {
    pub id: Id,
    pub name: Name,
    pub price: Price,
    pub metadata: metadata::Metadata,
}
