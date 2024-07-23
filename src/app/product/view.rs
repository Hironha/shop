use domain::extra;
use domain::product;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct ProductView<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub price: u64,
    pub extras: Vec<ExtraView<'a>>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl<'a> ProductView<'a> {
    pub fn new(product: &'a product::Product) -> Self {
        use rust_decimal::prelude::ToPrimitive;

        // TODO: maybe converting to cents is not really a good idea
        let price = product.price.decimal().to_u64().unwrap_or_default();
        let price_cents = price * 100;

        Self {
            id: product.id().uuid(),
            name: product.name.as_str(),
            price: price_cents,
            extras: product
                .extras
                .as_slice()
                .iter()
                .map(ExtraView::new)
                .collect(),
            created_at: product.metadata.created_at(),
            updated_at: product.metadata.updated_at(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ExtraView<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub price: u64,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl<'a> ExtraView<'a> {
    pub fn new(extra: &'a extra::Extra) -> Self {
        use rust_decimal::prelude::ToPrimitive;

        // TODO: maybe converting to cents is not really a good idea
        let price = extra.price.decimal().to_u64().unwrap_or_default();
        let cents = price * 100;
        Self {
            id: extra.id().uuid(),
            name: extra.name.as_str(),
            price: cents,
            created_at: extra.metadata.created_at(),
            updated_at: extra.metadata.updated_at(),
        }
    }
}
