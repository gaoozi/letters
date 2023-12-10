use sqlx::{pool::PoolOptions, Pool, Postgres};
use std::{env, sync::Arc};

pub type Db = Arc<Pool<Postgres>>;

pub async fn new_db_pool() -> Pool<Postgres> {
    let max_connections = 5;
    let db_url = env::var("DATABASE_URL").expect("Can't found DATABASE_URL.");

    PoolOptions::new()
        .max_connections(max_connections)
        .connect(&db_url)
        .await
        .expect("Fail to create pool")
    // .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
