mod dto;

pub use dto::{CreateInput, DeleteInput, ExtrasIds, FindInput, UpdateInput};

use domain::catalog;
use domain::extra;
use domain::product;

#[derive(Clone, Debug)]
pub struct ProductService<T, U> {
    products: T,
    extras: U,
}

impl<T: product::Repository, U: extra::Repository> ProductService<T, U> {
    pub fn new(products: T, extras: U) -> Self {
        Self { products, extras }
    }
}

impl<T: product::Repository, U: extra::Repository> ProductService<T, U> {
    pub async fn create(&mut self, input: CreateInput) -> Result<product::Product, product::Error> {
        let catalog_id = catalog::Id::parse_str(&input.catalog_id)?;
        let name = product::Name::new(input.name)?;
        let kind = product::Kind::parse_str(&input.kind)?;
        let extras_ids = parse_extras_ids(&input.extras_ids.take())?;

        let extras = self.find_extras(&extras_ids).await?;

        let product_extras = product::Extras::new(extras)?;
        let product = product::Product::new(
            catalog_id,
            name,
            product::Price::from_cents(input.price),
            kind,
            product_extras,
        );

        self.products.create(&product).await?;

        Ok(product)
    }

    pub async fn delete(&mut self, input: DeleteInput) -> Result<product::Product, product::Error> {
        let id = product::Id::parse_str(&input.id)?;
        let catalog_id = catalog::Id::parse_str(&input.catalog_id)?;

        self.products.delete(id, catalog_id).await
    }

    pub async fn find(&self, input: FindInput) -> Result<product::Product, product::Error> {
        let id = product::Id::parse_str(&input.id)?;
        let catalog_id = catalog::Id::parse_str(&input.catalog_id)?;

        self.products.find(id, catalog_id).await
    }

    pub async fn update(&mut self, input: UpdateInput) -> Result<product::Product, product::Error> {
        let id = product::Id::parse_str(&input.id)?;
        let name = product::Name::new(input.name)?;
        let catalog_id = catalog::Id::parse_str(&input.catalog_id)?;
        let kind = product::Kind::parse_str(&input.kind)?;
        let extras_ids = parse_extras_ids(&input.extras_ids.take())?;

        let mut product = self.products.find(id, catalog_id).await?;

        let extras = self.find_extras(&extras_ids).await?;
        let product_extras = product::Extras::new(extras)?;

        product.name = name;
        product.price = product::Price::from_cents(input.price);
        product.kind = kind;
        product.extras = product_extras;
        product.metadata.update();

        self.products.update(&product).await?;

        Ok(product)
    }

    async fn find_extras(
        &self,
        extras_ids: &[extra::Id],
    ) -> Result<Vec<extra::Extra>, product::Error> {
        self.extras
            .find_many(extras_ids)
            .await
            .map_err(|err| match err {
                extra::Error::NotFound(id) => product::Error::extra_not_found(id),
                err => product::Error::any(err),
            })
    }
}

fn parse_extras_ids(extras_ids: &[String]) -> Result<Vec<extra::Id>, product::Error> {
    extras_ids
        .iter()
        .map(|id| extra::Id::parse_str(id))
        .collect::<Result<Vec<extra::Id>, extra::IdError>>()
        .map_err(product::Error::from)
}
