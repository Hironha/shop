#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// TODO: remove later
#![allow(dead_code)]

mod app;
mod infra;

use axum::routing;
use axum::Router;
use infra::InMemSessions;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::app::catalog::api as catalog_api;
use crate::app::extra::api as extra_api;
use crate::app::product::api as product_api;
use crate::app::user::api as user_api;

#[derive(Clone, Debug)]
pub struct Context {
    pool: PgPool,
    sessions: InMemSessions,
}

#[tokio::main]
async fn main() {
    // TODO: config db url with environment variable
    let db_url = "postgresql://root:root@localhost:5432/shop";

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(db_url)
        .await
        .expect("Connection to postgres database");

    let context = Context {
        pool,
        sessions: InMemSessions::new(),
    };

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/register", routing::post(user_api::register))
                .route("/login", routing::post(user_api::login))
                .route(
                    "/catalogs",
                    routing::get(catalog_api::list).post(catalog_api::create),
                )
                .route(
                    "/catalogs/:id",
                    routing::delete(catalog_api::delete)
                        .get(catalog_api::find)
                        .put(catalog_api::update),
                )
                .route(
                    "/catalogs/:catalog_id/products",
                    routing::post(product_api::create),
                )
                .route(
                    "/catalogs/:catalog_id/products/:id",
                    routing::delete(product_api::delete)
                        .get(product_api::find)
                        .put(product_api::update),
                )
                .route(
                    "/extras",
                    routing::get(extra_api::all).post(extra_api::create),
                )
                .route(
                    "/extras/:id",
                    routing::delete(extra_api::delete).put(extra_api::update),
                ),
        )
        .with_state(context);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
