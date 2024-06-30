use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use domain::product;
use serde::Deserialize;

use super::service::{CreateInput, DeleteInput, ExtrasIds, FindInput, ProductService, UpdateInput};
use super::view::ProductView;
use crate::app::ApiError;
use crate::infra::{PgExtras, PgProducts};
use crate::Context;

#[derive(Clone, Debug, Deserialize)]
pub struct CreatePath {
    pub catalog_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateBody {
    pub name: String,
    pub price: u64,
    pub extras_ids: Vec<String>,
}

pub async fn create(
    State(ctx): State<Context>,
    Path(path): Path<CreatePath>,
    Json(body): Json<CreateBody>,
) -> impl IntoResponse {
    let extras_ids = match ExtrasIds::new(body.extras_ids) {
        Ok(extras_ids) => extras_ids,
        Err(err) => return create_error_response(product::Error::from(err)).into_response(),
    };

    let input = CreateInput {
        catalog_id: path.catalog_id,
        name: body.name,
        price: body.price,
        extras_ids,
    };

    let mut service =
        ProductService::new(PgProducts::new(ctx.pool.clone()), PgExtras::new(ctx.pool));

    let created_product = match service.create(input).await {
        Ok(product) => product,
        Err(err) => {
            eprintln!("Create product error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(ProductView::new(&created_product)).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeletePath {
    pub id: String,
    pub catalog_id: String,
}

pub async fn delete(State(ctx): State<Context>, Path(path): Path<DeletePath>) -> impl IntoResponse {
    let input = DeleteInput {
        id: path.id,
        catalog_id: path.catalog_id,
    };

    let mut service =
        ProductService::new(PgProducts::new(ctx.pool.clone()), PgExtras::new(ctx.pool));

    let deleted_product = match service.delete(input).await {
        Ok(product) => product,
        Err(err) => {
            eprintln!("Delete product error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(ProductView::new(&deleted_product)).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct FindPath {
    pub id: String,
    pub catalog_id: String,
}

pub async fn find(State(ctx): State<Context>, Path(path): Path<FindPath>) -> impl IntoResponse {
    let input = FindInput {
        id: path.id,
        catalog_id: path.catalog_id,
    };

    let service = ProductService::new(PgProducts::new(ctx.pool.clone()), PgExtras::new(ctx.pool));

    let found_product = match service.find(input).await {
        Ok(product) => product,
        Err(err) => {
            eprintln!("Find product error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(ProductView::new(&found_product)).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdatePath {
    pub id: String,
    pub catalog_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateBody {
    pub name: String,
    pub price: u64,
    pub extras_ids: Vec<String>,
}

pub async fn update(
    State(ctx): State<Context>,
    Path(path): Path<UpdatePath>,
    Json(body): Json<UpdateBody>,
) -> impl IntoResponse {
    let extras_ids = match ExtrasIds::new(body.extras_ids) {
        Ok(extras_ids) => extras_ids,
        Err(err) => return create_error_response(product::Error::from(err)).into_response(),
    };

    let input = UpdateInput {
        id: path.id,
        catalog_id: path.catalog_id,
        name: body.name,
        price: body.price,
        extras_ids,
    };

    let pg_products = PgProducts::new(ctx.pool.clone());
    let pg_extras = PgExtras::new(ctx.pool);
    let mut service = ProductService::new(pg_products, pg_extras);

    let updated_product = match service.update(input).await {
        Ok(product) => product,
        Err(err) => {
            eprintln!("Update product error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(ProductView::new(&updated_product)).into_response()
}

pub fn create_error_response(err: product::Error) -> impl IntoResponse {
    use product::Error;

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
