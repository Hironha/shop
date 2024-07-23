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

    pub async fn create(
        &mut self,
        input: CreateInput,
    ) -> Result<catalog::CatalogProducts, catalog::Error> {
        let name = catalog::Name::new(input.name)?;
        let description = input
            .description
            .map(catalog::Description::new)
            .transpose()?;

        let catalog = catalog::Catalog::new(name, description);
        self.catalogs.create(&catalog).await?;

        let products = catalog::Products::default();
        Ok(catalog::CatalogProducts::new(catalog, products))
    }

    pub async fn delete(
        &mut self,
        input: DeleteInput,
    ) -> Result<catalog::CatalogProducts, catalog::Error> {
        let id = catalog::Id::parse_str(&input.id)?;
        self.catalogs.delete(id).await
    }

    pub async fn find(&self, input: FindInput) -> Result<catalog::CatalogProducts, catalog::Error> {
        let id = catalog::Id::parse_str(&input.id)?;
        self.catalogs.find(id).await
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
        let id = catalog::Id::parse_str(&input.id)?;
        let name = catalog::Name::new(input.name)?;
        let description = input
            .description
            .map(catalog::Description::new)
            .transpose()?;

        let mut catalog_products = self.catalogs.find(id).await?;
        catalog_products.catalog.name = name;
        catalog_products.catalog.description = description;
        catalog_products.catalog.set_updated();

        self.catalogs.update(&catalog_products.catalog).await?;

        Ok(catalog::CatalogProducts::new(
            catalog_products.catalog,
            catalog_products.products,
        ))
    }
}
