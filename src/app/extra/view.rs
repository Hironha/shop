use askama::Template;
use serde::Serialize;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use domain::extra::Extra;

#[derive(Clone, Debug, Serialize)]
pub struct ExtraView<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub price: u64,
    pub created_at: String,
    pub updated_at: String,
}

impl<'a> ExtraView<'a> {
    pub fn new(extra: &'a Extra) -> Self {
        use rust_decimal::prelude::ToPrimitive;

        let metadata = extra.metadata();
        // TODO: maybe converting to cents is not really a good idea
        let price = extra.price().decimal().to_u64().unwrap_or_default();
        let cents = price * 100;

        Self {
            id: extra.id().uuid(),
            name: extra.name().as_str(),
            price: cents,
            created_at: metadata.created_at().format(&Rfc3339).unwrap_or_default(),
            updated_at: metadata.updated_at().format(&Rfc3339).unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/extras/index.j2")]
pub struct ListTempl<'a> {
    pub extras: Vec<ExtraView<'a>>,
}

impl<'a> ListTempl<'a> {
    pub fn try_to_html(self) -> Result<String, Box<dyn std::error::Error>> {
        self.render().map_err(Box::from)
    }
}
