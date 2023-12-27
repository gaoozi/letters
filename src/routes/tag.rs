use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use crate::{
    app::AppState,
    error::Result,
    helper::jwt::AuthClaims,
    models::{
        article::PreviewArticle,
        tag::{Tag, TagData},
        Pag, PagRsp,
    },
    repositories::{article::ArticleRepo, tag::TagRepo, Repositories},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_tags))
        .route("/", post(create_tag))
        .route("/:tag_name", get(get_preview_articles_by_tag))
}

async fn get_tags(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Tag>>> {
    let tags = state.repo.tag().get_all().await?;
    Ok(Json(tags))
}

async fn get_preview_articles_by_tag(
    State(state): State<Arc<AppState>>,
    Path(tag_name): Path<String>,
    Json(pag): Json<Pag>,
) -> Result<Json<PagRsp<PreviewArticle>>> {
    let articles = state
        .repo
        .article()
        .get_list_by_tag(&tag_name, &pag)
        .await?;
    Ok(Json(articles))
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
