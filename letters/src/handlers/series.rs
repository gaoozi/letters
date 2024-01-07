use crate::{
    app::AppState,
    dto::{
        series::{SeriesRequest, SeriesResponse, UpdateSeriesRequest},
        PageQueryParam,
    },
    error::{AppError, AppResult, Resource, ResourceType},
    repos::series,
    utils::jwt::AuthClaims,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::sync::Arc;

/// Create series.
#[utoipa::path(
    post,
    path = "/api/v1/series",
    request_body = SeriesRequest,
    responses(
        (status = 200, description = "Success create series", body = [()]),
        (status = 400, description = "Invalid data input", body = [AppError]),
        (status = 500, description = "Internal server error", body = [AppError])
    ),
    security(("jwt" = []))
)]
pub async fn create_series(
    claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<SeriesRequest>,
) -> AppResult<Json<()>> {
    series::create(&state.dbc, claims.user_id, &req).await?;
    Ok(Json(()))
}

pub async fn update_series(
    _claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
    Json(req): Json<UpdateSeriesRequest>,
) -> AppResult<Json<()>> {
    series::update(&state.dbc, series_id, &req).await?;
    Ok(Json(()))
}

pub async fn get_series(
    State(state): State<Arc<AppState>>,
    Query(param): Query<PageQueryParam>,
) -> AppResult<Json<Vec<SeriesResponse>>> {
    let resp = series::read_all(&state.dbc, &param)
        .await?
        .into_iter()
        .map(SeriesResponse::from)
        .collect();
    Ok(Json(resp))
}

pub async fn get_series_by_id(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
) -> AppResult<Json<SeriesResponse>> {
    let model = series::read_by_id(&state.dbc, series_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(Resource {
                r#type: ResourceType::Series,
                detail: "Not found this series.".to_string(),
            })
        })?;
    Ok(Json(SeriesResponse::from(model)))
}

pub async fn delete_series(
    _claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
) -> AppResult<Json<()>> {
    Ok(Json(series::delete_by_id(&state.dbc, series_id).await?))
}
