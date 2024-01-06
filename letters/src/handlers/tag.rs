use crate::{
    app::AppState,
    dto::tag::{TagRequest, TagResponse, UpdateTagRequest},
    error::{AppError, AppResult, Resource, ResourceType},
    repos::tag,
    utils::jwt::AuthClaims,
};
use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

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
