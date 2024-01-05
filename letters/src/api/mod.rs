mod handlers;
mod request;
mod response;
mod v1;

use crate::app::AppState;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().nest("/v1", v1::router())
}
