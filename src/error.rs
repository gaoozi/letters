use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::models;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Model(#[from] models::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!(
            { "message": self.to_string() }
        ));
        (StatusCode::OK, body).into_response()
    }
}
