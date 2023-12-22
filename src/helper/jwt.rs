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
use secrecy::{ExposeSecret, Secret};
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
    pub fn to_jwt(&self, secret: &Secret<String>) -> Result<String> {
        encode(self.user_id, secret.expose_secret())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub exp: usize, // Expiration time (as UTC timestamp). validate_exp defaults to true in validation
    pub iat: usize, // Issued at (as UTC timestamp)
    pub user_id: i32,
}

impl AuthClaims {
    fn new(user_id: i32) -> Self {
        Self {
            exp: (chrono::Local::now() + chrono::Duration::days(30)).timestamp() as usize,
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

        let token_data = decode(bearer.token(), state.secret.expose_secret())
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        Ok(token_data.claims)
    }
}

pub fn encode(user_id: i32, secret: &str) -> Result<String> {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let claims = AuthClaims::new(user_id);

    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| Error::Auth(AuthError::TokenCreation))
}

pub fn decode(token: &str, secret: &str) -> Result<TokenData<AuthClaims>> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    jsonwebtoken::decode(token, &decoding_key, &Validation::default())
        .map_err(|_| Error::Auth(AuthError::InvalidToken))
}
