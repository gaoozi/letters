use crate::{
    api::{
        request::login::LoginRequest,
        response::{login::LoginResponse, TokenClaims},
    },
    app::AppState,
    helper::hash::verify_password,
};
use axum::{extract::State, http::StatusCode, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let ents = entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(&payload.username))
        .all(state.dbc.as_ref())
        .await;

    match ents {
        Ok(users) => {
            if users.is_empty() {
                return Err(StatusCode::UNAUTHORIZED);
            }

            let user = &users[0];
            if verify_password(&payload.password, &user.password_hash).is_err() {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    let claims = TokenClaims {
        sub: payload.username,
        exp: (chrono::Utc::now() + chrono::Duration::seconds(state.conf.auth.timeout_seconds))
            .timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.conf.auth.secret.as_bytes()),
    )
    .unwrap();

    Ok(Json(LoginResponse {
        status: "success".to_string(),
        token: token,
    }))
}
