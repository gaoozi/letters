use secrecy::Secret;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub status: String,
    pub token: String,
}
