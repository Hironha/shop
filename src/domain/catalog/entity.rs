use super::{Description, Id, Name};
use crate::metadata;

#[derive(Clone, Debug)]
pub struct Catalog {
    pub(super) id: Id,
    pub name: Name,
    pub description: Option<Description>,
    pub(super) metadata: metadata::Metadata,
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
    pub fn config(config: Config) -> Self {
        Self {
            id: config.id,
            name: config.name,
            description: config.description,
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
    pub description: Option<Description>,
    pub metadata: metadata::Metadata,
}
