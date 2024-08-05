mod dto;

pub use dto::{CreateInput, DeleteInput, FindInput, ListInput, UpdateInput};

use domain::catalog;

#[derive(Clone, Debug)]
pub struct CatalogService<T> {
    catalogs: T,
}

impl<T: catalog::Repository> CatalogService<T> {
    pub fn new(catalogs: T) -> Self {
        Self { catalogs }
    }
}

impl<T: catalog::Repository> CatalogService<T> {
    pub async fn create(
        &mut self,
        input: CreateInput,
    ) -> Result<catalog::ProductCatalog, catalog::Error> {
        let catalog = catalog::Catalog::new(input.name, input.description);
        self.catalogs.create(&catalog).await?;

        let products = catalog::Products::default();
        Ok(catalog::ProductCatalog::new(catalog, products))
    }

    pub async fn delete(
        &mut self,
        input: DeleteInput,
    ) -> Result<catalog::ProductCatalog, catalog::Error> {
        self.catalogs.delete(input.id).await
    }

    pub async fn find(&self, input: FindInput) -> Result<catalog::ProductCatalog, catalog::Error> {
        self.catalogs.find(input.id).await
    }

    pub async fn list(&self, input: ListInput) -> Result<catalog::Pagination, catalog::Error> {
        let query = catalog::ListQuery {
            page: input.page,
            limit: input.limit,
        };

        self.catalogs.list(query).await
    }

    pub async fn update(
        &mut self,
        input: UpdateInput,
    ) -> Result<catalog::ProductCatalog, catalog::Error> {
        let mut product_catalog = self.catalogs.find(input.id).await?;
        product_catalog.catalog.name = input.name;
        product_catalog.catalog.description = input.description;
        product_catalog.catalog.metadata.update();

        self.catalogs.update(&product_catalog.catalog).await?;

        Ok(catalog::ProductCatalog::new(
            product_catalog.catalog,
            product_catalog.products,
        ))
    }
}
