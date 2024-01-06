use crate::{
    app::AppState,
    dto::category::{CategoryRequest, CategoryResponse, UpdateCategoryRequest},
    error::{AppError, AppResult, Resource, ResourceType},
    repos::category,
    utils::jwt::AuthClaims,
};
use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

pub async fn create_category(
    _claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CategoryRequest>,
) -> AppResult<Json<()>> {
    let model = category::check_name_exist(&state.dbc, &req.name).await?;
    if model.is_some() {
        return Err(AppError::ResourceExistsError(Resource {
            r#type: ResourceType::Category,
            detail: "Category name already exists".to_string(),
        }));
    }

    category::create(&state.dbc, &req).await?;
    Ok(Json(()))
}

pub async fn update_category(
    _claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<i32>,
    Json(req): Json<UpdateCategoryRequest>,
) -> AppResult<Json<()>> {
    if req.name.is_some() {
        let model = category::check_name_exist(&state.dbc, &req.name.clone().unwrap()).await?;
        if model.is_some() && category_id != model.unwrap().id {
            return Err(AppError::ResourceExistsError(Resource {
                r#type: ResourceType::Category,
                detail: "Category name already exists".to_string(),
            }));
        }
    }

    category::update(&state.dbc, category_id, &req).await?;
    Ok(Json(()))
}

pub async fn get_categories(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<CategoryResponse>>> {
    let resp = category::read_all(&state.dbc)
        .await?
        .into_iter()
        .map(CategoryResponse::from)
        .collect();
    Ok(Json(resp))
}
