use axum::extract::State;
use axum::response::{Html, IntoResponse, Response};

use super::service::ExtraService;
use super::view::{ExtraView, ListTempl};
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
    let templ = ListTempl { extras: views };
    // TODO: remove unwrap
    Html(templ.try_to_html().unwrap()).into_response()
}
