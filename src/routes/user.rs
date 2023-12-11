use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::error::Result;
use crate::helper::jwt::AuthClaims;
use crate::models::user::UpdateUser;
use crate::{
    app::AppState,
    models::user::{NewUser, User, UserBody},
    repositories::{user::UserRepo, Repositories},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_current_user).put(update_user))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>> {
    let user = state.repo.user().create(req.user).await?;
    Ok(Json(UserBody { user }))
}

async fn get_current_user(
    auth_user: AuthClaims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserBody<User>>> {
    let user = state.repo.user().get(auth_user.user_id).await?;
    Ok(Json(UserBody { user }))
}

async fn update_user(
    auth_user: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UserBody<UpdateUser>>,
) -> Result<Json<UserBody<User>>> {
    if req.user == UpdateUser::default() {
        return get_current_user(auth_user, State(state)).await;
    }

    let user = state
        .repo
        .user()
        .update(auth_user.user_id, req.user)
        .await?;
    Ok(Json(UserBody { user }))
}
