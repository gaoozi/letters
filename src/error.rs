use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("fail to hash password")]
    FailToHashPassword(#[from] argon2::password_hash::Error),

    #[error("{0}")]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!(
            { "message": self.to_string() }
        ));
        (StatusCode::OK, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Missing authentication credentials")]
    MissingCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication token")]
    InvalidToken,
}
