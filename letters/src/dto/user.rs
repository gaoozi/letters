use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Debug)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, IntoParams)]
pub struct ResetPassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserProfile {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
}

impl From<entity::user::Model> for UserProfile {
    fn from(value: entity::user::Model) -> Self {
        UserProfile {
            username: value.username,
            email: value.email,
            bio: value.bio,
            avatar: value.avatar,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Default, IntoParams)]
#[serde(default)]
pub struct UpdateUserProfile {
    pub username: Option<String>,
    // pub email: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}
