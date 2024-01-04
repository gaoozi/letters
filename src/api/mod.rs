mod handlers;
mod v1;

use std::sync::Arc;

use axum::Router;

use crate::app::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().nest("/v1", v1::router())
}
