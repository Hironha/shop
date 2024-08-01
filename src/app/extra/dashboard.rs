use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use serde::Deserialize;

use domain::extra;

use super::service::{CreateInput, DeleteInput, ExtraService, FindInput, UpdateInput};
use super::view::{DeleteModalTempl, ExtraView, ListTempl, UpdateModalView};
use crate::infra::PgExtras;
use crate::Context;

pub async fn all(State(ctx): State<Context>) -> Response {
    let service = ExtraService::new(PgExtras::new(ctx.pool));
    let extras = match service.all().await {
        Ok(extras) => extras,
        Err(err) => {
            eprintln!("All products extras error: {err:?}");
            // TODO: map error into html response
            todo!()
            // return create_error_response(err).into_response();
        }
    };

    let views = extras.iter().map(ExtraView::new).collect::<Vec<_>>();
    let templ = ListTempl::new(views);
    // TODO: remove unwrap
    Html(templ.try_to_html().unwrap()).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateBody {
    pub name: String,
    pub price: String,
}

pub async fn create(State(ctx): State<Context>, Form(mut body): Form<CreateBody>) -> Response {
    // TODO: map error into html response
    let name = extra::Name::new(body.name).unwrap();

    body.price.retain(|c| char::is_digit(c, 10));
    // TODO: map error into html response
    let price = body.price.parse().map(extra::Price::from_cents).unwrap();
    let input = CreateInput { name, price };

    let mut service = ExtraService::new(PgExtras::new(ctx.pool));
    // TODO: map error into html response
    let _ = service.create(input).await.unwrap();
    let headers = [("Hx-Redirect", "/dashboard/extras")];

    (StatusCode::CREATED, headers).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeletePath {
    pub id: String,
}

pub async fn delete(State(ctx): State<Context>, Path(path): Path<DeletePath>) -> Response {
    // TODO: map error into html response
    let id = extra::Id::parse_str(&path.id).unwrap();
    let input = DeleteInput { id };

    let mut service = ExtraService::new(PgExtras::new(ctx.pool));
    // TODO: map error into html response
    let _deleted = service.delete(input).await.unwrap();
    let headers = [("HX-Redirect", "/dashboard/extras")];

    (StatusCode::NO_CONTENT, headers).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteViewPath {
    pub id: String,
}

pub async fn delete_view(State(ctx): State<Context>, Path(path): Path<DeleteViewPath>) -> Response {
    // TODO: map error into html response
    let id = extra::Id::parse_str(&path.id).unwrap();
    let input = FindInput { id };

    let service = ExtraService::new(PgExtras::new(ctx.pool));
    // TODO: map error into html response
    let extra = service.find(input).await.unwrap();

    let templ = DeleteModalTempl::new(ExtraView::new(&extra));
    // TODO: map error into html response
    Html(templ.try_to_html().unwrap()).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdatePath {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateBody {
    pub name: String,
    pub price: String,
}

pub async fn update(
    State(ctx): State<Context>,
    Path(path): Path<UpdatePath>,
    Form(mut body): Form<UpdateBody>,
) -> Response {
    // TODO: map error into html response
    let id = extra::Id::parse_str(&path.id).unwrap();
    // TODO: map error into html response
    let name = extra::Name::new(body.name).unwrap();

    body.price.retain(|c| char::is_digit(c, 10));
    // TODO: map error into html response
    let price = body.price.parse().map(extra::Price::from_cents).unwrap();
    let input = UpdateInput { id, name, price };

    let mut service = ExtraService::new(PgExtras::new(ctx.pool));
    // TODO: map error into html response
    let _updated = service.update(input).await.unwrap();

    let headers = [("Hx-Redirect", "/dashboard/extras")];

    (StatusCode::OK, headers).into_response()
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateViewPath {
    id: String,
}

pub async fn update_view(State(ctx): State<Context>, Path(path): Path<UpdateViewPath>) -> Response {
    // TODO: map error into html response
    let id = extra::Id::parse_str(&path.id).unwrap();
    let input = FindInput { id };

    let service = ExtraService::new(PgExtras::new(ctx.pool));
    // TODO: map error into html response
    let extra = service.find(input).await.unwrap();
    let templ = UpdateModalView::new(ExtraView::new(&extra));

    // TODO: map error into html response
    Html(templ.try_to_html().unwrap()).into_response()
}
