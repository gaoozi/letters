mod app;
mod error;
mod helper;
mod log;
mod models;
mod repositories;
mod routes;

use dotenvy::dotenv;
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file not found");

    let _guards = log::setup();
    // let db = models::new().await?;

    app::serve().await;
    Ok(())
}
