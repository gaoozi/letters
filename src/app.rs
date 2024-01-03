use std::sync::Arc;

use axum::Router;
use secrecy::Secret;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{
    conf::Conf,
    repositories::{create_repositories, RepoImpls},
    routes,
};

pub struct AppState {
    pub secret: Secret<String>,
    pub repo: RepoImpls,
}

pub async fn serve(port: u16, conf: &Conf) {
    let state = Arc::new(AppState {
        repo: create_repositories().await,
        secret: Secret::new(conf.auth.secret.to_owned()),
    });

    let app = Router::new()
        .nest("/api", routes::api_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(Arc::clone(&state));

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
