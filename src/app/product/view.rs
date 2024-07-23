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
        Self {
            id: product.id().uuid(),
            name: product.name.as_str(),
            // TODO: maybe converting to cents is not really a good idea
            price: product.price.to_cents(),
            extras: product.extras.iter().map(ExtraView::new).collect(),
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
        Self {
            id: extra.id().uuid(),
            name: extra.name.as_str(),
            // TODO: maybe converting to cents is not really a good idea
            price: extra.price.to_cents(),
            created_at: extra.metadata.created_at(),
            updated_at: extra.metadata.updated_at(),
        }
    }
}
