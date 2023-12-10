mod error;
// mod store;
pub mod user;

pub use self::error::{Error, Result};
// use self::store::new_db_pool;
// pub use self::store::Db;

// pub async fn new() -> Result<Db> {
//     let db = new_db_pool().await?;
//     Ok(db)
// }
