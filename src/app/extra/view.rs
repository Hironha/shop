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
    extras: Vec<ExtraView<'a>>,
}

impl<'a> ListTempl<'a> {
    pub fn new(extras: Vec<ExtraView<'a>>) -> Self {
        Self { extras }
    }
}

impl<'a> ListTempl<'a> {
    pub fn try_to_html(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.render().map_err(Box::from)
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/extras/delete-modal.j2")]
pub struct DeleteModalTempl<'a> {
    extra: ExtraView<'a>,
}

impl<'a> DeleteModalTempl<'a> {
    pub fn new(extra: ExtraView<'a>) -> Self {
        Self { extra }
    }
}

impl<'a> DeleteModalTempl<'a> {
    pub fn try_to_html(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.render().map_err(Box::from)
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/extras/update-modal.j2")]
pub struct UpdateModalView<'a> {
    extra: ExtraView<'a>,
}

impl<'a> UpdateModalView<'a> {
    pub fn new(extra: ExtraView<'a>) -> Self {
        Self { extra }
    }
}

impl<'a> UpdateModalView<'a> {
    pub fn try_to_html(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.render().map_err(Box::from)
    }
}
