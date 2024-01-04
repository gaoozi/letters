use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::app::AppState;

use super::handlers;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/ping", get(handlers::ping::ping))
        .route("/login", post(handlers::login::login))
}
