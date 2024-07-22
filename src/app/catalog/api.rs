use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use domain::catalog;

use super::service::{CatalogService, CreateInput, DeleteInput, FindInput, ListInput, UpdateInput};
use super::view::{CatalogProductsView, PaginationView};
use crate::app::ApiError;
use crate::infra::PgCatalogs;
use crate::Context;

#[derive(Clone, Debug, Deserialize)]
pub struct CreateBody {
    pub name: String,
    pub description: Option<String>,
}

pub async fn create(State(ctx): State<Context>, Json(body): Json<CreateBody>) -> Response {
    let input = CreateInput {
        name: body.name,
        description: body.description,
    };

    let mut service = CatalogService::new(PgCatalogs::new(ctx.pool));
    let created_product_catalog = match service.create(input).await {
        Ok(product_catalog) => product_catalog,
        Err(err) => {
            eprintln!("Create product catalog error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(CatalogProductsView::new(&created_product_catalog)).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeletePath {
    pub id: String,
}

pub async fn delete(State(ctx): State<Context>, Path(path): Path<DeletePath>) -> Response {
    let input = DeleteInput { id: path.id };
    let mut service = CatalogService::new(PgCatalogs::new(ctx.pool));
    let deleted_product_catalog = match service.delete(input).await {
        Ok(product_catalog) => product_catalog,
        Err(err) => {
            eprintln!("Delete product catalog error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(CatalogProductsView::new(&deleted_product_catalog)).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct FindPath {
    pub id: String,
}

pub async fn find(State(ctx): State<Context>, Path(path): Path<FindPath>) -> Response {
    let input = FindInput { id: path.id };
    let service = CatalogService::new(PgCatalogs::new(ctx.pool));
    let found_product_catalog = match service.find(input).await {
        Ok(product_catalog) => product_catalog,
        Err(err) => {
            eprintln!("Find product catalog error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(CatalogProductsView::new(&found_product_catalog)).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

pub async fn list(State(ctx): State<Context>, Query(query): Query<ListQuery>) -> Response {
    let input = ListInput {
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(10),
    };

    let service = CatalogService::new(PgCatalogs::new(ctx.pool));
    let pagination = match service.list(input).await {
        Ok(pagination) => pagination,
        Err(err) => {
            eprintln!("List product catalogs error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    let view = PaginationView::new(&pagination);
    Json(view).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateBody {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdatePath {
    pub id: String,
}

pub async fn update(
    State(ctx): State<Context>,
    Path(path): Path<UpdatePath>,
    Json(body): Json<UpdateBody>,
) -> Response {
    let input = UpdateInput {
        id: path.id,
        name: body.name,
        description: body.description,
    };

    let mut service = CatalogService::new(PgCatalogs::new(ctx.pool));
    let updated_product_catalog = match service.update(input).await {
        Ok(product_catalog) => product_catalog,
        Err(err) => {
            eprintln!("Update product catalog error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(CatalogProductsView::new(&updated_product_catalog)).into_response()
}

fn create_error_response(err: catalog::Error) -> impl IntoResponse {
    use catalog::Error;

    match err {
        Error::Conflict(kind) => (
            StatusCode::CONFLICT,
            Json(ApiError::new("Conflict", kind.to_string())),
        ),
        Error::Internal(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Internal", "Internal server error")),
        ),
        Error::NotFound(kind) => (
            StatusCode::NOT_FOUND,
            Json(ApiError::new("NotFound", kind.to_string())),
        ),
        Error::Validation(kind) => (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Validation", kind.to_string())),
        ),
    }
}
