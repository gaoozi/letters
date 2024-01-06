use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    app::AppState,
    error::{AppError, AppResult},
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthClaims {
    pub sub: i32,
    pub iat: usize, // issued at
    pub exp: usize, // expiration
}

impl AuthClaims {
    pub fn new(sub: i32, expire_time: i64) -> Self {
        let now = chrono::Utc::now();
        Self {
            sub,
            iat: now.timestamp() as usize,
            exp: (now + chrono::Duration::seconds(expire_time)).timestamp() as usize,
        }
    }

    pub fn encode(&self, secret: &str) -> AppResult<String> {
        let encoding_key = EncodingKey::from_secret(secret.as_ref());

        jsonwebtoken::encode(&Header::default(), self, &encoding_key).map_err(AppError::Jwt)
    }

    pub fn decode(token: &str, secret: &str) -> AppResult<TokenData<Self>> {
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        jsonwebtoken::decode(token, &decoding_key, &Validation::default()).map_err(AppError::Jwt)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(AppError::TypeHeader)?;

        let state = Arc::<AppState>::from_ref(state);

        // Decode the user data
        let token_data = AuthClaims::decode(bearer.token(), &state.conf.auth.secret)?;
        Ok(token_data.claims)
    }
}

/*
use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    app::AppState,
    error::{AuthError, Error, Result},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: i32,
}

impl AuthUser {
    pub fn to_jwt(&self, secret: &str, timeout_seconds: i64) -> Result<String> {
        encode(self.user_id, secret, timeout_seconds)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub exp: usize, // Expiration time (as UTC timestamp). validate_exp defaults to true in validation
    pub iat: usize, // Issued at (as UTC timestamp)
    pub user_id: i32,
}

impl AuthClaims {
    fn new(user_id: i32, timeout_seconds: i64) -> Self {
        Self {
            exp: (chrono::Local::now() + chrono::Duration::seconds(timeout_seconds)).timestamp()
                as usize,
            iat: chrono::Local::now().timestamp() as usize,
            user_id,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> core::result::Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        let state = Arc::<AppState>::from_ref(state);

        let token_data = decode(bearer.token(), &state.conf.auth.secret)
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        Ok(token_data.claims)
    }
}

pub fn encode(user_id: i32, secret: &str, timeout_seconds: i64) -> Result<String> {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let claims = AuthClaims::new(user_id, timeout_seconds);

    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| Error::Auth(AuthError::TokenCreation))
}

pub fn decode(token: &str, secret: &str) -> Result<TokenData<AuthClaims>> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    jsonwebtoken::decode(token, &decoding_key, &Validation::default())
        .map_err(|_| Error::Auth(AuthError::InvalidToken))
}

*/
