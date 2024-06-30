use askama::Template;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use domain::extra::Extra;

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
    pub fn new(entity: &'a Extra) -> Self {
        use rust_decimal::prelude::ToPrimitive;

        // TODO: maybe converting to cents is not really a good idea
        let price = entity.price().decimal().to_u64().unwrap_or_default();
        let cents = price * 100;
        Self {
            id: entity.id().uuid(),
            name: entity.name().as_str(),
            price: cents,
            created_at: entity.metadata().created_at(),
            updated_at: entity.metadata().updated_at(),
        }
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/extras.j2")]
pub struct ListTempl<'a> {
    pub extras: Vec<ExtraView<'a>>,
}

impl<'a> ListTempl<'a> {
    pub fn try_to_html(self) -> Result<String, Box<dyn std::error::Error>> {
        self.render().map_err(Box::from)
    }
}
