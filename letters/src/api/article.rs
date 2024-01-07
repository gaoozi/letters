use axum::{routing::get, Router};
use std::sync::Arc;

use crate::{app::AppState, handlers};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(handlers::article::get_articles).post(handlers::article::create_article),
        )
        .route(
            "/:article_id",
            get(handlers::article::get_article_by_id)
                .put(handlers::article::update_article)
                .delete(handlers::article::delete_article),
        )
}
