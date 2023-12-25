use std::sync::Arc;

use axum::Router;

use crate::app::AppState;

mod category;
mod tag;
mod user;

pub fn api_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/users", user::router())
        .nest("/categories", category::router())
        .nest("/tags", tag::router())
}
