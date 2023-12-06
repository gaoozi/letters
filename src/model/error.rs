use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Store(#[from] store::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

//impl std::fmt::Display for Error {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "{self:?}")
//    }
//}

//impl std::error::Error for Error {}
