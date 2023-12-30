use std::sync::Arc;

use axum::Router;
use secrecy::Secret;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{
    conf::Conf,
    repositories::{create_repositories, RepoImpls},
    routes::api_router,
};

pub struct AppState {
    pub secret: Secret<String>,
    pub repo: RepoImpls,
}

pub async fn serve(conf: Conf) {
    let state = Arc::new(AppState {
        repo: create_repositories().await,
        secret: Secret::new(conf.auth.secret),
    });

    let app = Router::new()
        .nest("/api", api_router())
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
