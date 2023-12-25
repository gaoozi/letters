use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserBody<T> {
    pub token: Option<String>,
    pub user: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub avatar: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub last_seen: Option<NaiveDateTime>,
    pub is_active: Option<i8>,
}

#[derive(Serialize, Debug)]
pub struct Profile {
    pub name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Default, PartialEq, Eq)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ResetPassword {
    pub old_password: String,
    pub new_password: String,
}
