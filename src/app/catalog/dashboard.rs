use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse};
use serde::Deserialize;

use super::service::{CatalogService, ListInput};
use super::view::{ListPageTempl, ListTempl, PaginationView};
use crate::infra::PgCatalogs;
use crate::Context;

#[derive(Clone, Debug, Deserialize)]
pub enum ListKind {
    #[serde(alias = "full")]
    Full,
    #[serde(alias = "table")]
    Table,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub kind: Option<ListKind>,
}

pub async fn list(State(ctx): State<Context>, Query(query): Query<ListQuery>) -> impl IntoResponse {
    let input = ListInput {
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(10),
    };

    let service = CatalogService::new(PgCatalogs::new(ctx.pool));
    let pagination = match service.list(input).await {
        Ok(pagination) => pagination,
        Err(err) => {
            eprintln!("List product catalogs error: {err:?}");
            // TODO: create error with html response
            todo!()
            // return create_error_response(err).into_response();
        }
    };

    let view = PaginationView::new(&pagination);
    let html = match query.kind.unwrap_or(ListKind::Full) {
        ListKind::Full => ListPageTempl { pagination: view }.to_html(),
        ListKind::Table => ListTempl { pagination: view }.to_html(),
    };

    Html(html).into_response()
}
