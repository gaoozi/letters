use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{app::AppState, handlers};

use super::user;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/authorize", post(handlers::auth::authorize))
        .nest("/users", user::router())
}
