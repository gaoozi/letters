use crate::helper;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    UtilError(#[from] helper::error::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
