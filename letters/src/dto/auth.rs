use serde::Deserialize;

#[derive(Deserialize, ToSchema)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}
