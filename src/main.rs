mod error;
mod log;
mod model;

use axum::{response::Html, routing::get, Router};
use dotenvy::dotenv;
use error::Result;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::model::ModelManager;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file not found");

    let _guards = log::setup();
    let _mm = ModelManager::new().await?;

    let app = Router::new().route("/", get(handler)).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!")
}
