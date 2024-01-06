use std::fmt::Debug;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use strum::EnumString;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(thiserror::Error)]
pub enum AppError {
    #[error("{0} not found")]
    NotFound(Resource),
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    TypeHeader(#[from] axum_extra::typed_header::TypedHeaderRejection),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl AppError {
    fn response(&self) -> (StatusCode, ErrorResponse) {
        match self {
            AppError::NotFound(resource) => (
                StatusCode::NOT_FOUND,
                ErrorResponse::new("".to_string(), resource.detail.clone(), self.to_string()),
            ),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new(
                    "".to_string(),
                    self.to_string(),
                    String::from("Database error."),
                ),
            ),
            AppError::TypeHeader(_) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse::new(
                    "".to_string(),
                    self.to_string(),
                    String::from("Invalid token"),
                ),
            ),
            AppError::Jwt(_) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse::new(
                    "".to_string(),
                    self.to_string(),
                    String::from("Unauthorized error."),
                ),
            ),
            AppError::Unexpected(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("".to_string(), self.to_string(), self.to_string()),
            ),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body) = self.response();
        (status_code, Json(body)).into_response()
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub error: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: String, error: String, message: String) -> Self {
        Self {
            code,
            error,
            message,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub r#type: ResourceType,
    pub detail: String,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.r#type.fmt(f)
    }
}

#[derive(Debug, Clone, EnumString)]
pub enum ResourceType {
    #[strum(serialize = "User")]
    User,
    Tag,
    Category,
    Article,
    Series,
    Comment,
}
