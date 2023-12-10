use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserBody<T> {
    pub user: T,
}

#[derive(Deserialize, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUser {
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
pub struct ResetPassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub token: String,
    pub bio: String,
    pub avatar: Option<String>,
}
