pub mod catalog;
pub mod extra;
pub mod product;
pub mod user;

use serde::Serialize;
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ApiError {
    code: String,
    message: String,
    #[serde(with = "time::serde::rfc3339")]
    time: OffsetDateTime,
}

impl ApiError {
    pub(crate) fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            time: OffsetDateTime::now_utc(),
        }
    }

    pub(crate) fn validation(err: &impl std::error::Error) -> Self {
        Self::new("Validation", err.to_string())
    }

    pub(crate) fn internal() -> Self {
        Self::new("Internal", "Internal server error")
    }
}
