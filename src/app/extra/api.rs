use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use domain::extra;

use super::service::{CreateInput, DeleteInput, ExtraService, UpdateInput};
use super::view::ExtraView;
use crate::app::ApiError;
use crate::infra::PgExtras;
use crate::Context;

pub async fn all(State(ctx): State<Context>) -> Response {
    let service = ExtraService::new(PgExtras::new(ctx.pool));
    let extras = match service.all().await {
        Ok(extras) => extras,
        Err(err) => {
            eprintln!("All products extras error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    let views = extras.iter().map(ExtraView::new).collect::<Vec<_>>();
    Json(views).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateBody {
    pub name: String,
    pub price: u64,
}

pub async fn create(State(ctx): State<Context>, Json(body): Json<CreateBody>) -> Response {
    let input = CreateInput {
        name: body.name,
        price: body.price,
    };

    let mut service = ExtraService::new(PgExtras::new(ctx.pool));
    let created_product_extra = match service.create(input).await {
        Ok(product_extra) => product_extra,
        Err(err) => {
            eprintln!("Create product extra error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(ExtraView::new(&created_product_extra)).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeletePath {
    pub id: String,
}

pub async fn delete(State(ctx): State<Context>, Path(path): Path<DeletePath>) -> Response {
    let input = DeleteInput { id: path.id };
    let mut service = ExtraService::new(PgExtras::new(ctx.pool));
    let deleted_product_extra = match service.delete(input).await {
        Ok(product_extra) => product_extra,
        Err(err) => {
            eprintln!("Delete product extra error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(ExtraView::new(&deleted_product_extra)).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdatePath {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateBody {
    pub name: String,
    pub price: u64,
}

pub async fn update(
    State(ctx): State<Context>,
    Path(path): Path<UpdatePath>,
    Json(body): Json<UpdateBody>,
) -> Response {
    let input = UpdateInput {
        id: path.id,
        name: body.name,
        price: body.price,
    };

    let mut service = ExtraService::new(PgExtras::new(ctx.pool));
    let updated_product_extra = match service.update(input).await {
        Ok(product_extra) => product_extra,
        Err(err) => {
            eprintln!("Update product extra error: {err:?}");
            return create_error_response(err).into_response();
        }
    };

    Json(ExtraView::new(&updated_product_extra)).into_response()
}

fn create_error_response(err: extra::Error) -> impl IntoResponse {
    use extra::Error;

    match err {
        Error::Conflict(kind) => (
            StatusCode::CONFLICT,
            Json(ApiError::new("Conflict", kind.to_string())),
        ),
        Error::Internal(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Internal", "Internal server error")),
        ),
        Error::NotFound(_) => (
            StatusCode::NOT_FOUND,
            Json(ApiError::new("NotFound", err.to_string())),
        ),
        Error::Validation(kind) => (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Validation", kind.to_string())),
        ),
    }
}
