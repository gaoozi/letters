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
            get(handlers::tag::get_tags).post(handlers::tag::create_tag),
        )
        .route("/:tag_id/articles", get(handlers::tag::get_tag_articles))
        .route(
            "/:tag_id",
            put(handlers::tag::update_tag).delete(handlers::tag::delete_tag),
        )
}
