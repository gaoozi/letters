use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Store(#[from] store::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
