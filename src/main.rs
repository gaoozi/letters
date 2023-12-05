pub mod log;

use axum::{response::Html, routing::get, Router};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let _guards = log::setup();

    let app = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!")
}
