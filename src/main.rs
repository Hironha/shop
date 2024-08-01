#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// TODO: remove later
#![allow(dead_code)]

mod app;
mod infra;

use askama::Template;
use axum::routing;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::app::catalog::api as catalog_api;
use crate::app::catalog::dashboard as catalog_dashboard;
use crate::app::extra::api as extra_api;
use crate::app::extra::dashboard as extra_dashboard;
use crate::app::product::api as product_api;

#[derive(Clone, Debug)]
pub struct Context {
    pool: PgPool,
}

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/health-check.j2")]
pub struct HelloTempl;

#[derive(Clone, Debug, Template)]
#[template(path = "./pages/not-found.j2")]
pub struct NotFoundTempl;

#[tokio::main]
async fn main() {
    // TODO: config db url with environment variable
    let db_url = "postgresql://root:root@localhost:5432/shop";

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(db_url)
        .await
        .expect("Connection to postgres database");

    let context = Context { pool };

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
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
        .nest(
            "/dashboard",
            Router::new()
                .route("/assets/main.css", routing::get(serve_css))
                .route("/catalogs", routing::get(catalog_dashboard::list))
                .route(
                    "/extras",
                    routing::get(extra_dashboard::all).post(extra_dashboard::create),
                )
                .route(
                    "/extras/:id",
                    routing::delete(extra_dashboard::delete).put(extra_dashboard::update),
                )
                .route(
                    "/extra/:id/update",
                    routing::get(extra_dashboard::update_view),
                )
                .route(
                    "/extras/:id/delete",
                    routing::get(extra_dashboard::delete_view),
                )
                .route("/health-check", routing::get(health_check))
                .fallback(routing::any(not_found)),
        )
        .with_state(context);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl axum::response::IntoResponse {
    let html = HelloTempl;
    axum::response::Html(html.render().unwrap())
}

async fn not_found() -> impl axum::response::IntoResponse {
    let html = NotFoundTempl;
    axum::response::Html(html.render().unwrap())
}

// TODO: replace by tower service to serve static files
async fn serve_css() -> impl axum::response::IntoResponse {
    use axum::http::header;

    let bytes = include_bytes!("../dashboard/assets/main.css");
    let body = axum::body::Bytes::from_static(bytes);
    let mut headers = header::HeaderMap::new();
    headers.append(
        header::CONTENT_TYPE,
        "text/css; charset=utf-8".parse().unwrap(),
    );

    (headers, body)
}
