use axum::{
    routing::{get, put},
    Router,
};
use std::sync::Arc;

use crate::{app::AppState, handlers};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/profile",
            get(handlers::user::get_profile).put(handlers::user::update_profile),
        )
        .route("/password", put(handlers::user::reset_password))
}
