use serde::Deserialize;
use sqlx::types::{Decimal, Json, Uuid};
use sqlx::FromRow;
use time::OffsetDateTime;

use domain::catalog;
use domain::extra;
use domain::metadata;
use domain::product;

use crate::infra::extra::ExtraModel;

#[derive(Clone, Debug, Deserialize, FromRow)]
pub struct ProductModel {
    pub id: Uuid,
    pub catalog_id: Uuid,
    pub name: String,
    pub price: Decimal,
    pub extras: Json<Vec<ExtraModel>>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl ProductModel {
    // TODO: find a better way to map model into entity
    pub fn try_into_entity(self) -> Result<product::Product, product::Error> {
        let name = product::Name::new(self.name)?;
        let extras = self
            .extras
            .0
            .into_iter()
            .map(ExtraModel::try_into_entity)
            .collect::<Result<Vec<extra::Extra>, extra::Error>>()
            .map_err(product::Error::any)?;

        let extras = product::Extras::new(extras)?;

        let metadata = metadata::Metadata::configured(self.created_at, self.updated_at)
            .map_err(product::Error::any)?;

        let product = product::Product::config(product::Config {
            id: product::Id::from(self.id),
            catalog_id: catalog::Id::from(self.catalog_id),
            name,
            price: product::Price::from(self.price),
            extras: Some(extras),
            metadata,
        });

        Ok(product)
    }
}
