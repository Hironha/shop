use askama::Template;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use domain::catalog;

use crate::app::product::view::ProductView;

#[derive(Clone, Debug, Serialize)]
pub struct CatalogView<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub products: Vec<ProductView<'a>>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl<'a> CatalogView<'a> {
    pub fn new(entity: &'a catalog::Catalog) -> Self {
        let products = entity.products().as_slice();
        Self {
            id: entity.id().uuid(),
            name: entity.name().as_str(),
            description: entity.description().map(catalog::Description::as_str),
            products: products.iter().map(ProductView::new).collect(),
            created_at: entity.metadata().created_at(),
            updated_at: entity.metadata().updated_at(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PaginationView<'a> {
    pub count: u64,
    pub page: u64,
    pub limit: u64,
    pub items: Vec<CatalogView<'a>>,
}

impl<'a> PaginationView<'a> {
    pub fn new(pagination: &'a catalog::Pagination) -> Self {
        Self {
            count: pagination.count,
            page: pagination.page,
            limit: pagination.limit,
            items: pagination.items.iter().map(CatalogView::new).collect(),
        }
    }

    pub fn try_into_html(self) -> Result<String, Box<dyn std::error::Error>> {
        let templ = ListTempl { pagination: self };
        templ.render().map_err(Box::from)
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/catalogs.j2")]
struct ListTempl<'a> {
    pub pagination: PaginationView<'a>,
}
