use super::{Description, Error, Id, Name, Products};
use crate::metadata;
use crate::product;

#[derive(Clone, Debug)]
pub struct Catalog {
    pub(super) id: Id,
    pub(super) name: Name,
    pub(super) description: Option<Description>,
    pub(super) products: Products,
    pub(super) metadata: metadata::Metadata,
}

impl Catalog {
    #[must_use]
    pub fn new(name: Name, description: Option<Description>, products: Products) -> Self {
        Self {
            id: Id::new(),
            name,
            description,
            products,
            metadata: metadata::Metadata::new(),
        }
    }

    #[must_use]
    pub fn config(config: Config) -> Self {
        Self {
            id: config.id,
            name: config.name,
            description: config.description,
            products: config.products,
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
    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    #[must_use]
    pub fn products(&self) -> &Products {
        &self.products
    }

    #[must_use]
    pub fn metadata(&self) -> &metadata::Metadata {
        &self.metadata
    }

    /// Try adding `product` as a catalog product
    ///
    /// # Errors
    ///
    /// Returns an [`Err`] if catalog cannot have more products registered
    pub fn try_add_product(&mut self, product: product::Product) -> Result<(), Error> {
        self.products.try_push(product).map_err(Error::from)
    }

    #[must_use]
    pub fn into_setter(self) -> Setter {
        Setter(self)
    }
}

impl From<Config> for Catalog {
    fn from(value: Config) -> Self {
        Self::config(value)
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub id: Id,
    pub name: Name,
    pub description: Option<Description>,
    pub products: Products,
    pub metadata: metadata::Metadata,
}

#[derive(Clone, Debug)]
pub struct Setter(Catalog);

impl Setter {
    #[must_use]
    pub fn name(mut self, name: Name) -> Self {
        self.0.name = name;
        self
    }

    #[must_use]
    pub fn description(mut self, description: Option<Description>) -> Self {
        self.0.description = description;
        self
    }

    #[must_use]
    pub fn products(mut self, products: Products) -> Self {
        self.0.products = products;
        self
    }

    #[must_use]
    pub fn commit(mut self) -> Catalog {
        self.0.metadata.update();
        self.0
    }
}
