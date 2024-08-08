use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use domain::user;

use super::service::{LoginInput, LogoutInput, RegisterInput, UserService};
use crate::app::ApiError;
use crate::infra::{Argon2Encrypter, LettreMailer, PgSessions, PgUsers};
use crate::Context;

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

    let pg_users = PgUsers::new(ctx.pool.clone());
    let pg_sessions = PgSessions::new(ctx.pool);
    let mut service = UserService::new(pg_users, pg_sessions, Argon2Encrypter::new(), LettreMailer);
    // TODO: map error into response
    let user_id = service.login(input).await.unwrap();
    println!("{user_id:?}");

    (StatusCode::OK).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct LogoutBody {
    pub user_id: String,
}

pub async fn logout(State(ctx): State<Context>, Json(body): Json<LogoutBody>) -> Response {
    let user_id = match user::Id::parse_str(&body.user_id) {
        Ok(session_id) => session_id,
        Err(err) => return create_validation_error_response(&err),
    };
    let input = LogoutInput { user_id };

    let pg_users = PgUsers::new(ctx.pool.clone());
    let pg_sessions = PgSessions::new(ctx.pool);
    let mut service = UserService::new(pg_users, pg_sessions, Argon2Encrypter::new(), LettreMailer);
    // TODO: map error into response
    service.logout(input).await.unwrap();

    (StatusCode::OK).into_response()
}

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

    let pg_users = PgUsers::new(ctx.pool.clone());
    let pg_sessions = PgSessions::new(ctx.pool);
    let mut service = UserService::new(pg_users, pg_sessions, Argon2Encrypter::new(), LettreMailer);
    // TODO: map error into response
    service.register(input).await.unwrap();

    (StatusCode::CREATED).into_response()
}

fn create_validation_error_response(err: &dyn std::error::Error) -> Response {
    eprintln!("Validation error: {err:?}");
    let body = ApiError::new("Validation", err.to_string());
    (StatusCode::BAD_GATEWAY, Json(body)).into_response()
}
