mod dto;

pub use dto::{CreateInput, DeleteInput, ExtrasIds, FindInput, UpdateInput};

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
        let found_extras = self.find_extras(input.extras_ids.as_slice()).await?;
        let extras = product::Extras::new(found_extras).map_err(product::Error::any)?;

        let product = product::Product::new(
            input.catalog_id,
            input.name,
            input.price,
            input.kind,
            extras,
        );

        self.products.create(&product).await?;

        Ok(product)
    }

    pub async fn delete(&mut self, input: DeleteInput) -> Result<product::Product, product::Error> {
        self.products.delete(input.id, input.catalog_id).await
    }

    pub async fn find(&self, input: FindInput) -> Result<product::Product, product::Error> {
        self.products.find(input.id, input.catalog_id).await
    }

    pub async fn update(&mut self, input: UpdateInput) -> Result<product::Product, product::Error> {
        let mut product = self.products.find(input.id, input.catalog_id).await?;

        let extras = self.find_extras(input.extras_ids.as_slice()).await?;
        let product_extras = product::Extras::new(extras).map_err(product::Error::any)?;

        product.name = input.name;
        product.price = input.price;
        product.kind = input.kind;
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
