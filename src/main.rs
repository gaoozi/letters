mod app;
mod conf;
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
    let conf = conf::init().expect("Initial config failed!");

    app::serve(conf).await;
    Ok(())
}
