use super::{Catalog, Products};

#[derive(Clone, Debug)]
pub struct CatalogProducts {
    pub catalog: Catalog,
    pub products: Products,
}

impl CatalogProducts {
    #[must_use]
    pub fn new(catalog: Catalog, products: Products) -> Self {
        Self { catalog, products }
    }
}
