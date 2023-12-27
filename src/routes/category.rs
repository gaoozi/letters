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
        category::{Category, CategoryData},
        Pag, PagRsp,
    },
    repositories::{article::ArticleRepo, category::CategoryRepo, Repositories},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_categories))
        .route("/", post(create_category))
        .route("/:category_name", get(get_preview_articles_by_category))
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

async fn get_preview_articles_by_category(
    State(state): State<Arc<AppState>>,
    Path(category_name): Path<String>,
    Json(pag): Json<Pag>,
) -> Result<Json<PagRsp<PreviewArticle>>> {
    let articles = state
        .repo
        .article()
        .get_list_by_category(&category_name, &pag)
        .await?;
    Ok(Json(articles))
}
