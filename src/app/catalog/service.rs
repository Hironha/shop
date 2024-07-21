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

    pub async fn create(&mut self, input: CreateInput) -> Result<catalog::Catalog, catalog::Error> {
        let name = catalog::Name::new(input.name)?;
        let description = input
            .description
            .map(catalog::Description::new)
            .transpose()?;

        let products = catalog::Products::default();
        let catalog = catalog::Catalog::new(name, description, products);

        self.catalogs.create(&catalog).await?;

        Ok(catalog)
    }

    pub async fn delete(&mut self, input: DeleteInput) -> Result<catalog::Catalog, catalog::Error> {
        let id = catalog::Id::parse_str(&input.id)?;
        self.catalogs.delete(id).await
    }

    pub async fn find(&self, input: FindInput) -> Result<catalog::Catalog, catalog::Error> {
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

    pub async fn update(&mut self, input: UpdateInput) -> Result<catalog::Catalog, catalog::Error> {
        let id = catalog::Id::parse_str(&input.id)?;
        let name = catalog::Name::new(input.name)?;
        let description = input
            .description
            .map(catalog::Description::new)
            .transpose()?;

        let catalog = self.catalogs.find(id).await?;
        let updated_catalog = catalog
            .into_setter()
            .name(name)
            .description(description)
            .commit();

        self.catalogs.update(&updated_catalog).await?;

        Ok(updated_catalog)
    }
}
