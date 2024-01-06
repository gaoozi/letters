use crate::{
    app::AppState,
    dto::auth::{AuthRequest, AuthResponse},
    error::{AppError, AppResult, Resource, ResourceType},
    repos,
    utils::{hash::verify_password, jwt::AuthClaims},
};
use axum::{extract::State, Json};
use std::sync::Arc;

pub async fn authorize(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> AppResult<Json<AuthResponse>> {
    let user = repos::user::read_by_email(&state.dbc, &payload.email)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(Resource {
                r#type: ResourceType::User,
                detail: "Not found this user.".to_string(),
            })
        })?;

    verify_password(&payload.password, &user.password_hash)?;

    let token = AuthClaims::new(user.id, state.conf.auth.timeout_seconds)
        .encode(&state.conf.auth.secret)?;

    Ok(Json(AuthResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: state.conf.auth.timeout_seconds,
    }))
}
