use clap::Args;
use migration::{Migrator, MigratorTrait};
use sea_orm_migration::sea_orm::Database;

use crate::conf::Conf;

#[derive(Debug, Args)]
pub struct Cmd {}

pub fn handle(_cmd: &Cmd, conf: &Conf) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let db_url = conf.database.url.clone().unwrap_or("".to_string());
            let conn = Database::connect(db_url)
                .await
                .expect("Database connection failed");

            Migrator::up(&conn, None).await.unwrap();
        });

    Ok(())
}
