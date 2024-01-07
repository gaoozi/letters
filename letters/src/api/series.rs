use axum::{routing::get, Router};
use std::sync::Arc;

use crate::{app::AppState, handlers};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(handlers::series::get_series).post(handlers::series::create_series),
        )
        .route(
            "/:article_id",
            get(handlers::series::get_series_by_id)
                .put(handlers::series::update_series)
                .delete(handlers::series::delete_series),
        )
}
