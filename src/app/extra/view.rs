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
    pub created_at: String,
    pub updated_at: String,
}

impl<'a> ExtraView<'a> {
    pub fn new(extra: &'a Extra) -> Self {
        Self {
            id: extra.id().uuid(),
            name: extra.name.as_str(),
            price: extra.price.to_cents(),
            created_at: Self::to_rfc3339(extra.metadata.created_at()),
            updated_at: Self::to_rfc3339(extra.metadata.updated_at()),
        }
    }

    pub fn to_rfc3339(date: OffsetDateTime) -> String {
        use time::format_description::well_known::Rfc3339;
        date.format(&Rfc3339).unwrap_or_default()
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
