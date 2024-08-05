use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

use domain::catalog;
use domain::product;

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
    pub kind: String,
    pub extras_ids: Vec<String>,
}

pub async fn create(
    State(ctx): State<Context>,
    Path(path): Path<CreatePath>,
    Json(body): Json<CreateBody>,
) -> impl IntoResponse {
    let catalog_id = match catalog::Id::parse_str(&path.catalog_id) {
        Ok(catalog_id) => catalog_id,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let name = match product::Name::new(body.name) {
        Ok(name) => name,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let kind = match product::Kind::parse_str(&body.kind) {
        Ok(name) => name,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let extras_ids = match ExtrasIds::parse(&body.extras_ids) {
        Ok(extras_ids) => extras_ids,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let input = CreateInput {
        catalog_id,
        name,
        price: product::Price::from_cents(body.price),
        kind,
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
    let id = match product::Id::parse_str(&path.id) {
        Ok(id) => id,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let catalog_id = match catalog::Id::parse_str(&path.catalog_id) {
        Ok(catalog_id) => catalog_id,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let input = DeleteInput { id, catalog_id };

    let pg_products = PgProducts::new(ctx.pool.clone());
    let pg_extras = PgExtras::new(ctx.pool);

    let mut service = ProductService::new(pg_products, pg_extras);
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
    let id = match product::Id::parse_str(&path.id) {
        Ok(id) => id,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let catalog_id = match catalog::Id::parse_str(&path.id) {
        Ok(catalog_id) => catalog_id,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let input = FindInput { id, catalog_id };

    let pg_products = PgProducts::new(ctx.pool.clone());
    let pg_extras = PgExtras::new(ctx.pool);

    let service = ProductService::new(pg_products, pg_extras);
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
    pub kind: String,
    pub extras_ids: Vec<String>,
}

pub async fn update(
    State(ctx): State<Context>,
    Path(path): Path<UpdatePath>,
    Json(body): Json<UpdateBody>,
) -> impl IntoResponse {
    let id = match product::Id::parse_str(&path.id) {
        Ok(id) => id,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let catalog_id = match catalog::Id::parse_str(&path.catalog_id) {
        Ok(catalog_id) => catalog_id,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let name = match product::Name::new(body.name) {
        Ok(name) => name,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let kind = match product::Kind::parse_str(&body.kind) {
        Ok(kind) => kind,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let extras_ids = match ExtrasIds::parse(&body.extras_ids) {
        Ok(extras_ids) => extras_ids,
        Err(err) => return create_validation_error_response(&err).into_response(),
    };
    let input = UpdateInput {
        id,
        catalog_id,
        name,
        price: product::Price::from_cents(body.price),
        kind,
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
    }
}

fn create_validation_error_response(err: &dyn std::error::Error) -> impl IntoResponse {
    let body = ApiError::new("Validation", err.to_string());
    (StatusCode::BAD_GATEWAY, Json(body))
}
