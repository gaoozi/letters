use axum::{routing::get, Router};
use std::sync::Arc;

use crate::{app::AppState, handlers};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(handlers::series::get_all_series).post(handlers::series::create_series),
        )
        .route(
            "/:series_id/articles",
            get(handlers::series::get_series_articles),
        )
        .route(
            "/:series_id",
            get(handlers::series::get_series_by_id)
                .put(handlers::series::update_series)
                .delete(handlers::series::delete_series),
        )
}
