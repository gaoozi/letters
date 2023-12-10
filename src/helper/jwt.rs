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
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

use super::error::{AuthError, Error, Result};
use crate::app::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: sqlx::types::Uuid,
}

impl AuthUser {
    pub fn to_jwt(&self, secret: &Secret<String>) -> Result<String> {
        let claims = AuthClaims {
            user_id: self.user_id,
            exp: (chrono::Local::now() + chrono::Duration::days(30)).timestamp() as usize,
        };

        let key = Keys::new(secret.expose_secret().as_bytes());
        jsonwebtoken::encode(&Header::default(), &claims, &key.encoding)
            .map_err(|_| Error::Auth(AuthError::TokenCreation))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    user_id: sqlx::types::Uuid,
    exp: usize,
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
    ) -> std::result::Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        let state = Arc::<AppState>::from_ref(state);

        let key = Keys::new(state.secret.expose_secret().as_bytes());
        let token_data = decode(bearer.token(), &key.decoding, &Validation::default())
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        Ok(token_data.claims)
    }
}

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
