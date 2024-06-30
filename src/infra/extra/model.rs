use serde::Deserialize;
use sqlx::types::{Decimal, Uuid};
use sqlx::FromRow;
use time::OffsetDateTime;

use domain::extra;
use domain::metadata;

#[derive(Clone, Debug, Deserialize, FromRow)]
pub struct ExtraModel {
    pub id: Uuid,
    pub name: String,
    pub price: Decimal,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl ExtraModel {
    // TODO: find a better way to map model into entity
    pub fn try_into_entity(self) -> Result<extra::Extra, extra::Error> {
        let name = extra::Name::new(self.name)?;
        let metadata = metadata::Metadata::configured(self.created_at, self.updated_at)
            .map_err(|err| extra::Error::Internal(err.into()))?;

        let product_extra = extra::Extra::config(extra::Config {
            id: extra::Id::from(self.id),
            name,
            price: extra::Price::from_decimal(self.price),
            metadata,
        });

        Ok(product_extra)
    }
}
