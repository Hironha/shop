use askama::Template;
use serde::Serialize;
use time::format_description::well_known::Rfc3339;
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
        let products = value
            .products
            .as_slice()
            .iter()
            .map(ProductView::new)
            .collect();

        let metadata = value.catalog.metadata();

        Self {
            id: value.catalog.id().uuid(),
            name: value.catalog.name().as_str(),
            description: value
                .catalog
                .description()
                .map(catalog::Description::as_str),
            products,
            created_at: metadata.created_at().format(&Rfc3339).unwrap(),
            updated_at: metadata.updated_at().format(&Rfc3339).unwrap(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PaginationView<'a> {
    pub count: u64,
    pub page: u64,
    pub limit: u64,
    pub items: Vec<CatalogProductsView<'a>>,
}

impl<'a> PaginationView<'a> {
    pub fn new(pagination: &'a catalog::Pagination) -> Self {
        Self {
            count: pagination.count,
            page: pagination.page,
            limit: pagination.limit,
            items: pagination
                .items
                .iter()
                .map(CatalogProductsView::new)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/catalogs.j2")]
pub struct ListTempl<'a> {
    pub pagination: PaginationView<'a>,
}

impl<'a> ListTempl<'a> {
    pub fn try_into_html(self) -> Result<String, Box<dyn std::error::Error>> {
        self.render().map_err(Box::from)
    }
}
