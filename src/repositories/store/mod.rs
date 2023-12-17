use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{env, sync::Arc};

pub type Db = Arc<MySqlPool>;

pub async fn new_db_pool() -> MySqlPool {
    let max_connections = 5;
    let db_url = env::var("DATABASE_URL").expect("Can't found DATABASE_URL.");

    MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect(&db_url)
        .await
        .expect("Fail to create pool")
    // .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
