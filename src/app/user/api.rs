use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use domain::user;

use super::service::{LoginInput, RegisterInput, UserService};
use crate::app::ApiError;
use crate::infra::Argon2Encrypt;
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

    let mut service = UserService::new(Argon2Encrypt::new(), ctx.users);
    // TODO: map error into response
    service.register(input).await.unwrap();

    (StatusCode::CREATED).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LoginOut {
    pub token: String,
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

    let service = UserService::new(Argon2Encrypt::new(), ctx.users);
    // TODO: map error into response
    let token = service
        .login(input)
        .await
        .inspect_err(|err| println!("{err}"))
        .unwrap();

    (StatusCode::OK, Json(LoginOut { token })).into_response()
}

fn create_validation_error_response(err: &dyn std::error::Error) -> Response {
    let body = ApiError::new("Validation", err.to_string());
    (StatusCode::BAD_GATEWAY, Json(body)).into_response()
}
