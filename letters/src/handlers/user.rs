use crate::{
    app::AppState,
    dto::user::{ResetPassword, UpdateUserProfile, UserProfile},
    error::AppResult,
    repos::user,
    utils::{hash::verify_password, jwt::AuthClaims},
};
use axum::{extract::State, Json};
use std::sync::Arc;

/// Get user profile.
#[utoipa::path(
    get,
    path = "/api/v1/users/profile",
    responses(
        (status = 200, description = "Success get user profile", body = [UserProfile]),
        (status = 401, description = "Unauthorized user", body = [AppError]),
        (status = 500, description = "Internal server error", body = [AppError])
    ),
    security(("jwt" = []))
)]
pub async fn get_profile(
    claims: AuthClaims,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<UserProfile>> {
    let model = user::read_by_id(&state.dbc, claims.user_id).await?.unwrap();
    Ok(Json(UserProfile::from(model)))
}

/// Update user profile.
#[utoipa::path(
    put,
    path = "/api/v1/users/profile",
    request_body = UpdateUserProfile,
    responses(
        (status = 200, description = "Success update profile information", body = [()]),
        (status = 400, description = "Invalid data input", body = [AppError]),
        (status = 401, description = "Unauthorized user", body = [AppError]),
        (status = 500, description = "Internal server error", body = [AppError])
    ),
    security(("jwt" = []))
)]
pub async fn update_profile(
    claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateUserProfile>,
) -> AppResult<Json<()>> {
    user::update(&state.dbc, claims.user_id, &req).await?;
    Ok(Json(()))
}

/// Reset user password.
#[utoipa::path(
    put,
    path = "/api/v1/users/password",
    request_body = ResetPassword,
    responses(
        (status = 200, description = "Success update password login" , body = [()]),
        (status = 400, description = "Invalid data input", body = [AppError]),
        (status = 500, description = "Internal server error", body = [AppError])
    ),
    security(("jwt" = []))
)]
pub async fn reset_password(
    claims: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<ResetPassword>,
) -> AppResult<Json<()>> {
    let model = user::read_by_id(&state.dbc, claims.user_id).await?.unwrap();
    verify_password(&req.old_password, &model.password_hash)?;

    Ok(Json(
        user::update_password(&state.dbc, claims.user_id, &req.new_password).await?,
    ))
}
