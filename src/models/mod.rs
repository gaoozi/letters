mod error;
mod store;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

#[derive(Debug)]
pub struct ModelManager {
    _db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let _db = new_db_pool().await?;
        Ok(ModelManager { _db })
    }
}
