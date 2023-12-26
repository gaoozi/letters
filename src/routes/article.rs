use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::AppState,
    error::Result,
    helper::jwt::AuthClaims,
    models::{
        article::{Article, PreviewArticle},
        Pag, PagRsp,
    },
    repositories::{article::ArticleRepo, Repositories},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_preview_articles))
        .route("/:id", get(get_article))
        .route("/:id/read_count", put(update_read_count))
        .route("/:id/like_count", put(update_like_count))
}

async fn get_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Result<Json<Article>> {
    let article = state.repo.article().get_by_id(article_id).await?;
    Ok(Json(article.into_article()))
}

async fn get_preview_articles(
    State(state): State<Arc<AppState>>,
    Json(pag): Json<Pag>,
) -> Result<Json<PagRsp<PreviewArticle>>> {
    let articles = state.repo.article().get_list(&pag).await?;
    Ok(Json(articles))
}

async fn update_read_count(
    _author_user: AuthClaims,
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Result<()> {
    state.repo.article().update_read_count(article_id).await?;
    Ok(())
}

async fn update_like_count(
    _author_user: AuthClaims,
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Result<()> {
    state.repo.article().update_like_count(article_id).await?;
    Ok(())
}
