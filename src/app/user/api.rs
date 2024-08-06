use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use domain::user;

use super::service::{LoginInput, RegisterInput, UserService};
use crate::app::ApiError;
use crate::infra::{Argon2Encrypt, PgUsers};
use crate::Context;

#[derive(Clone, Debug, Deserialize)]
pub struct RegisterBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[allow(clippy::unused_async)]
pub async fn register(State(ctx): State<Context>, Json(body): Json<RegisterBody>) -> Response {
    let username = match user::Username::try_new(body.username) {
        Ok(username) => username,
        Err(err) => return create_validation_error_response(&err),
    };
    let email = match user::Email::try_new(body.email) {
        Ok(email) => email,
        Err(err) => return create_validation_error_response(&err),
    };
    let input = RegisterInput {
        username,
        email,
        password: body.password,
    };

    let pg_users = PgUsers::new(ctx.pool);
    let mut service = UserService::new(Argon2Encrypt::new(), pg_users, ctx.sessions);
    // TODO: map error into response
    service.register(input).await.unwrap();

    (StatusCode::CREATED).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

pub async fn login(State(ctx): State<Context>, Json(body): Json<LoginBody>) -> Response {
    let email = match user::Email::try_new(body.email) {
        Ok(email) => email,
        Err(err) => return create_validation_error_response(&err),
    };
    let input = LoginInput {
        email,
        password: body.password,
    };

    let pg_users = PgUsers::new(ctx.pool);
    let mut service = UserService::new(Argon2Encrypt::new(), pg_users, ctx.sessions);
    // TODO: map error into response
    let session_id = service.login(input).await.unwrap();
    println!("{session_id}");

    (StatusCode::OK).into_response()
}

fn create_validation_error_response(err: &dyn std::error::Error) -> Response {
    let body = ApiError::new("Validation", err.to_string());
    (StatusCode::BAD_GATEWAY, Json(body)).into_response()
}
