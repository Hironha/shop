use askama::Template;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use domain::catalog;

use crate::app::product::view::ProductView;

#[derive(Clone, Debug, Serialize)]
pub struct CatalogProductsView<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub products: Vec<ProductView<'a>>,
    pub created_at: String,
    pub updated_at: String,
}

impl<'a> CatalogProductsView<'a> {
    pub fn new(value: &'a catalog::CatalogProducts) -> Self {
        Self {
            id: value.catalog.id().uuid(),
            name: value.catalog.name.as_str(),
            description: value
                .catalog
                .description
                .as_ref()
                .map(catalog::Description::as_str),
            products: value.products.iter().map(ProductView::new).collect(),
            created_at: Self::to_rfc3339(value.catalog.metadata.created_at()),
            updated_at: Self::to_rfc3339(value.catalog.metadata.updated_at()),
        }
    }

    fn to_rfc3339(date: OffsetDateTime) -> String {
        use time::format_description::well_known::Rfc3339;
        date.format(&Rfc3339).unwrap_or_default()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PaginationView<'a> {
    pub count: u64,
    pub page: u32,
    pub limit: u8,
    pub items: Vec<CatalogProductsView<'a>>,
}

impl<'a> PaginationView<'a> {
    pub fn new(pagination: &'a catalog::Pagination) -> Self {
        Self {
            count: pagination.count,
            page: pagination.page.into(),
            limit: pagination.limit.into(),
            items: pagination
                .items
                .iter()
                .map(CatalogProductsView::new)
                .collect(),
        }
    }
}

impl<'a> PaginationView<'a> {
    pub fn pages(&self) -> Vec<u32> {
        let start = self.page.saturating_sub(2).max(1);
        let end = self.page.saturating_add(self.remaining_pages().min(2));
        (start..=end).collect()
    }

    pub fn has_previous_page(&self) -> bool {
        self.page > 1
    }

    pub fn has_next_page(&self) -> bool {
        self.remaining_pages() > 0
    }

    pub fn remaining_pages(&self) -> u32 {
        let pages = self.count.div_ceil(self.limit.into());
        let remaining = pages.saturating_sub(self.page.into());
        u32::try_from(remaining).unwrap_or(u32::MAX)
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/catalogs/index.j2")]
pub struct ListPageTempl<'a> {
    pub pagination: PaginationView<'a>,
}

impl<'a> ListPageTempl<'a> {
    pub fn to_html(&self) -> String {
        // TODO: remove unwrap
        self.render().unwrap()
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/catalogs/table.j2")]
pub struct ListTempl<'a> {
    pub pagination: PaginationView<'a>,
}

impl<'a> ListTempl<'a> {
    pub fn to_html(&self) -> String {
        // TODO: remove unwrap
        self.render().unwrap()
    }
}
