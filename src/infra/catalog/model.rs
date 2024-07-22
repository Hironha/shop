use serde::Deserialize;
use sqlx::types::{Json, Uuid};
use sqlx::FromRow;
use time::OffsetDateTime;

use domain::catalog;
use domain::metadata;

use crate::infra::product::ProductModel;

#[derive(Clone, Debug, FromRow, Deserialize)]
pub struct CatalogWithProductsModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub products: Json<Vec<ProductModel>>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl CatalogWithProductsModel {
    pub fn try_into_entity(
        self,
    ) -> Result<catalog::CatalogProducts, Box<dyn std::error::Error>> {
        let name = catalog::Name::new(self.name)?;
        let description = self
            .description
            .map(catalog::Description::new)
            .transpose()?;

        let products = self
            .products
            .0
            .into_iter()
            .map(ProductModel::try_into_entity)
            .collect::<Result<Vec<_>, _>>()?;

        let products = catalog::Products::new(products)?;

        let metadata = metadata::Metadata::configured(self.created_at, self.updated_at)?;
        let catalog = catalog::Catalog::config(catalog::Config {
            id: catalog::Id::from(self.id),
            name,
            description,
            metadata,
        });

        Ok(catalog::CatalogProducts::new(catalog, products))
    }
}
