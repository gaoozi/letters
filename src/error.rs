use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::error::DatabaseError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("fail to hash password")]
    FailToHashPassword(#[from] argon2::password_hash::Error),

    #[error("{0}")]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error("{0}")]
    UnprocessableEntity(String),
}

impl Error {
    pub fn code(&self) -> u32 {
        match self {
            Error::FailToHashPassword(_) => 1001,
            Error::Auth(_) => 2001,
            Error::Sqlx(_) => 3001,
            Error::UnprocessableEntity(_) => 4001,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!(
            { "body": self.code(), "message": self.to_string() }
        ));
        (StatusCode::OK, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    //#[error("Wrong authentication credentials")]
    //WrongCredentials,
    //#[error("Missing authentication credentials")]
    //MissingCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication token")]
    InvalidToken,
}

// A little helper trait for more easily converting database constraint errors into API errors.
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> core::result::Result<T, Error>;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> core::result::Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}
