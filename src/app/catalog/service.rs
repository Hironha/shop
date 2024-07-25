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
    ) -> Result<catalog::CatalogProducts, catalog::Error> {
        let catalog = catalog::Catalog::new(input.name, input.description);
        self.catalogs.create(&catalog).await?;

        let products = catalog::Products::default();
        Ok(catalog::CatalogProducts::new(catalog, products))
    }

    pub async fn delete(
        &mut self,
        input: DeleteInput,
    ) -> Result<catalog::CatalogProducts, catalog::Error> {
        self.catalogs.delete(input.id).await
    }

    pub async fn find(&self, input: FindInput) -> Result<catalog::CatalogProducts, catalog::Error> {
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
    ) -> Result<catalog::CatalogProducts, catalog::Error> {
        let mut catalog_products = self.catalogs.find(input.id).await?;
        catalog_products.catalog.name = input.name;
        catalog_products.catalog.description = input.description;
        catalog_products.catalog.metadata.update();

        self.catalogs.update(&catalog_products.catalog).await?;

        Ok(catalog::CatalogProducts::new(
            catalog_products.catalog,
            catalog_products.products,
        ))
    }
}
