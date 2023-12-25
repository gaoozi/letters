use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use crate::{
    app::AppState,
    error::Result,
    helper::jwt::AuthClaims,
    models::category::{Category, CategoryData},
    repositories::{category::CategoryRepo, Repositories},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_categories))
        .route("/", post(create_category))
}

async fn get_categories(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Category>>> {
    let categories = state.repo.category().get_all().await?;
    Ok(Json(categories))
}

async fn create_category(
    _auth_user: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CategoryData>,
) -> Result<Json<Category>> {
    let category_id = state.repo.category().create(req).await?;
    let category = state.repo.category().get(category_id as i32).await?;
    Ok(Json(category))
}
