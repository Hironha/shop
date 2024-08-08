use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use domain::user;

use super::service::{
    session, LoginError, LoginInput, LogoutError, LogoutInput, RegisterError, RegisterInput,
    UserService,
};
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
        Err(err) => return map_validation_error(&err),
    };
    let input = LoginInput {
        email,
        password: body.password,
    };

    let pg_users = PgUsers::new(ctx.pool.clone());
    let pg_sessions = PgSessions::new(ctx.pool);
    let mut service = UserService::new(pg_users, pg_sessions, Argon2Encrypter::new(), LettreMailer);
    let user_id = match service.login(input).await {
        Ok(user_id) => user_id,
        Err(err) => {
            eprintln!("Login error: {err:?}");
            return map_login_error(&err);
        }
    };

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
        Err(err) => return map_validation_error(&err),
    };
    let input = LogoutInput { user_id };

    let pg_users = PgUsers::new(ctx.pool.clone());
    let pg_sessions = PgSessions::new(ctx.pool);
    let mut service = UserService::new(pg_users, pg_sessions, Argon2Encrypter::new(), LettreMailer);
    if let Err(err) = service.logout(input).await {
        eprintln!("Logout error: {err:?}");
        return map_logout_error(&err);
    }

    (StatusCode::OK).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct RegisterBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(State(ctx): State<Context>, Json(body): Json<RegisterBody>) -> Response {
    let username = match user::Username::try_new(body.username) {
        Ok(username) => username,
        Err(err) => return map_validation_error(&err),
    };
    let email = match user::Email::try_new(body.email) {
        Ok(email) => email,
        Err(err) => return map_validation_error(&err),
    };
    let input = RegisterInput {
        username,
        email,
        password: body.password,
    };

    let pg_users = PgUsers::new(ctx.pool.clone());
    let pg_sessions = PgSessions::new(ctx.pool);
    let mut service = UserService::new(pg_users, pg_sessions, Argon2Encrypter::new(), LettreMailer);
    if let Err(err) = service.register(input).await {
        eprintln!("Register error: {err:?}");
        return map_register_error(&err);
    }

    (StatusCode::CREATED).into_response()
}

fn map_login_error(err: &LoginError) -> Response {
    match err {
        LoginError::Credentials => (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Credentials", err.to_string())),
        ),
        LoginError::Unverified => (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Unverified", err.to_string())),
        ),
        LoginError::User(user_err) => return map_user_error(user_err),
        LoginError::Session(session_err) => return map_session_error(session_err),
    }
    .into_response()
}

fn map_logout_error(err: &LogoutError) -> Response {
    match err {
        LogoutError::NotFound(_) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiError::new("Unauthorized", "User is not authorized")),
        ),
        LogoutError::Session(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal()),
        ),
    }
    .into_response()
}

fn map_register_error(err: &RegisterError) -> Response {
    match err {
        RegisterError::User(user::Error::EmailConflict(_)) => (
            StatusCode::CONFLICT,
            Json(ApiError::new("Conflict", err.to_string())),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal()),
        ),
    }
    .into_response()
}

fn map_user_error(err: &user::Error) -> Response {
    match err {
        user::Error::EmailConflict(_) => (
            StatusCode::CONFLICT,
            Json(ApiError::new("Conflict", err.to_string())),
        ),
        user::Error::EmailNotFound(_) => (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Credentials", "Invalid user credentials")),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal()),
        ),
    }
    .into_response()
}

fn map_session_error(err: &session::Error) -> Response {
    match err {
        session::Error::NotFound(_) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiError::new("Unauthorized", err.to_string())),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal()),
        ),
    }
    .into_response()
}

fn map_validation_error(err: &impl std::error::Error) -> Response {
    (StatusCode::BAD_GATEWAY, Json(ApiError::validation(err))).into_response()
}
