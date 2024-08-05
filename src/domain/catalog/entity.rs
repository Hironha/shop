use super::{Description, Id, Name};
use crate::core::metadata;

#[derive(Clone, Debug)]
pub struct Catalog {
    pub(super) id: Id,
    pub name: Name,
    pub description: Option<Description>,
    pub metadata: metadata::Metadata,
}

impl Catalog {
    #[must_use]
    pub fn new(name: Name, description: Option<Description>) -> Self {
        Self {
            id: Id::new(),
            name,
            description,
            metadata: metadata::Metadata::new(),
        }
    }

    #[must_use]
    pub fn config(config: CatalogConfig) -> Self {
        Self {
            id: config.id,
            name: config.name,
            description: config.description,
            metadata: config.metadata,
        }
    }
}

impl Catalog {
    #[must_use]
    pub fn id(&self) -> Id {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct CatalogConfig {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
    pub metadata: metadata::Metadata,
}
