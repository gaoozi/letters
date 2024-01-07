use crate::{
    app::AppState,
    dto::{
        article::{ArticleRequest, ArticleResponse, PreviewArticleResponse, UpdateArticleRequest},
        PageQueryParam,
    },
    error::{AppError, AppResult, Resource, ResourceType},
    repos::article,
    utils::jwt::AuthClaims,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::sync::Arc;

pub async fn create_article(
    claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<ArticleRequest>,
) -> AppResult<Json<()>> {
    article::create(&state.dbc, claims.user_id, &req).await?;
    Ok(Json(()))
}

pub async fn update_article(
    _claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
    Json(req): Json<UpdateArticleRequest>,
) -> AppResult<Json<()>> {
    article::update(&state.dbc, article_id, &req).await?;
    Ok(Json(()))
}

pub async fn get_articles(
    State(state): State<Arc<AppState>>,
    Query(param): Query<PageQueryParam>,
) -> AppResult<Json<Vec<PreviewArticleResponse>>> {
    let resp = article::read_all(&state.dbc, &param)
        .await?
        .into_iter()
        .map(PreviewArticleResponse::from)
        .collect();
    Ok(Json(resp))
}

pub async fn get_article_by_id(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> AppResult<Json<ArticleResponse>> {
    let model = article::read_by_id(&state.dbc, article_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(Resource {
                r#type: ResourceType::Article,
                detail: "Article found this user.".to_string(),
            })
        })?;

    Ok(Json(ArticleResponse::from(model)))
}

pub async fn delete_article(
    _claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> AppResult<Json<()>> {
    Ok(Json(article::delete_by_id(&state.dbc, article_id).await?))
}
