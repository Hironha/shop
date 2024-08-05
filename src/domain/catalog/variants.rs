use super::{Catalog, Products};

#[derive(Clone, Debug)]
pub struct ProductCatalog {
    pub catalog: Catalog,
    pub products: Products,
}

impl ProductCatalog {
    #[must_use]
    pub fn new(catalog: Catalog, products: Products) -> Self {
        Self { catalog, products }
    }
}
