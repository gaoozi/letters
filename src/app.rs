use std::sync::Arc;

use axum::{response::Html, routing::get};
use secrecy::Secret;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{
    repositories::{create_repositories, RepoImpls},
    routes::api_router,
};

pub struct AppState {
    // pub db: Db,
    pub secret: Secret<String>,
    pub repo: RepoImpls,
}

pub async fn serve() {
    let state = Arc::new(AppState {
        // db: db,
        repo: create_repositories().await,
        secret: Secret::new("".to_string()),
    });

    let app = api_router()
        .route("/", get(handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(Arc::clone(&state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!")
}
