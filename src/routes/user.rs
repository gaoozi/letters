use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::error::Result;
use crate::helper::jwt::{AuthClaims, AuthUser};
use crate::models::user::{LoginUser, UpdateUser};
use crate::{
    app::AppState,
    models::user::{NewUser, User, UserBody},
    repositories::{user::UserRepo, Repositories},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_current_user).put(update_user))
        .route("/login", post(login_user))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>> {
    let user_id = state.repo.user().create(req.user).await?;
    let user = state.repo.user().get(user_id as i32).await?;
    Ok(Json(UserBody { token: None, user }))
}

async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UserBody<LoginUser>>,
) -> Result<Json<UserBody<User>>> {
    let user = state
        .repo
        .user()
        .check(req.user.email, &req.user.password)
        .await?;

    Ok(Json(UserBody {
        token: Some(AuthUser { user_id: user.id }.to_jwt(&state.secret)?),
        user,
    }))
}

async fn get_current_user(
    auth_user: AuthClaims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserBody<User>>> {
    let user = state.repo.user().get(auth_user.user_id).await?;
    Ok(Json(UserBody { token: None, user }))
}

async fn update_user(
    auth_user: AuthClaims,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UserBody<UpdateUser>>,
) -> Result<Json<UserBody<User>>> {
    if req.user == UpdateUser::default() {
        return get_current_user(auth_user, State(state)).await;
    }

    state
        .repo
        .user()
        .update(auth_user.user_id, req.user)
        .await?;

    let user = state.repo.user().get(auth_user.user_id).await?;
    Ok(Json(UserBody { token: None, user }))
}
