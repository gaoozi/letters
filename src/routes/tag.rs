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
    models::tag::{Tag, TagData},
    repositories::{tag::TagRepo, Repositories},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_tags))
        .route("/", post(create_tag))
}

async fn get_tags(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Tag>>> {
    let tags = state.repo.tag().get_all().await?;
    Ok(Json(tags))
}

async fn create_tag(
    _auth_user: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<TagData>,
) -> Result<Json<Tag>> {
    let tag_id = state.repo.tag().create(req).await?;
    let tag = state.repo.tag().get(tag_id as i32).await?;
    Ok(Json(tag))
}
