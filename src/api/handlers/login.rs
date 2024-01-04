use crate::{
    app::AppState,
    helper::jwt::AuthUser,
    models::auth::{LoginRequest, LoginResponse},
};
use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(_payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user_id = 1;
    let token = AuthUser { user_id }
        .to_jwt(&state.conf.auth.secret, state.conf.auth.timeout_seconds)
        .unwrap();

    Ok(Json(LoginResponse {
        status: "success".to_string(),
        token,
    }))
}
