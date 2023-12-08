pub mod error;

use std::env;

use sqlx::{pool::PoolOptions, Pool, Postgres};

pub use self::error::{Error, Result};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    let max_connections = 5;
    let db_url = env::var("DATABASE_URL").expect("Can't found DATABASE_URL.");

    PoolOptions::new()
        .max_connections(max_connections)
        .connect(&db_url)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
