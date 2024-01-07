use crate::{
    app::AppState,
    dto::{
        article::PreviewArticleResponse,
        tag::{TagRequest, TagResponse, UpdateTagRequest},
        PageQueryParam,
    },
    error::{AppError, AppResult, Resource, ResourceType},
    repos::{article, tag},
    utils::jwt::AuthClaims,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::sync::Arc;

/// Create tag.
#[utoipa::path(
    post,
    path = "/api/v1/tags",
    request_body = TagRequest,
    responses(
        (status = 200, description = "Success update profile information", body = [()]),
        (status = 400, description = "Invalid data input", body = [AppError]),
        (status = 401, description = "Unauthorized user", body = [AppError]),
        (status = 500, description = "Internal server error", body = [AppError])
    ),
    security(("jwt" = []))
)]
pub async fn create_tag(
    _claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<TagRequest>,
) -> AppResult<Json<()>> {
    let model = tag::check_name_exist(&state.dbc, &req.name).await?;
    if model.is_some() {
        return Err(AppError::ResourceExistsError(Resource {
            r#type: ResourceType::Tag,
            detail: "Tag name already exists".to_string(),
        }));
    }

    tag::create(&state.dbc, &req).await?;
    Ok(Json(()))
}

pub async fn update_tag(
    _claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Path(tag_id): Path<i32>,
    Json(req): Json<UpdateTagRequest>,
) -> AppResult<Json<()>> {
    if req.name.is_some() {
        let model = tag::check_name_exist(&state.dbc, &req.name.clone().unwrap()).await?;
        if model.is_some() && tag_id != model.unwrap().id {
            return Err(AppError::ResourceExistsError(Resource {
                r#type: ResourceType::Tag,
                detail: "Tag name already exists".to_string(),
            }));
        }
    }

    tag::update(&state.dbc, tag_id, &req).await?;
    Ok(Json(()))
}

pub async fn get_tags(State(state): State<Arc<AppState>>) -> AppResult<Json<Vec<TagResponse>>> {
    let resp = tag::read_all(&state.dbc)
        .await?
        .into_iter()
        .map(TagResponse::from)
        .collect();
    Ok(Json(resp))
}

pub async fn get_tag_articles(
    State(state): State<Arc<AppState>>,
    Path(tag_id): Path<i32>,
    Query(param): Query<PageQueryParam>,
) -> AppResult<Json<Vec<PreviewArticleResponse>>> {
    let models = article::read_all_by_tag(&state.dbc, tag_id, &param)
        .await?
        .into_iter()
        .map(PreviewArticleResponse::from)
        .collect();
    Ok(Json(models))
}

pub async fn delete_tag(
    _claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Path(tag_id): Path<i32>,
) -> AppResult<Json<()>> {
    Ok(Json(tag::delete_by_id(&state.dbc, tag_id).await?))
}
