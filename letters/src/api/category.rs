use axum::{
    routing::{get, put},
    Router,
};
use std::sync::Arc;

use crate::{app::AppState, handlers};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(handlers::category::get_categories).post(handlers::category::create_category),
        )
        .route(
            "/:category_id/articles",
            get(handlers::category::get_category_articles),
        )
        .route(
            "/:category_id",
            put(handlers::category::update_category).delete(handlers::category::delete_category),
        )
}
