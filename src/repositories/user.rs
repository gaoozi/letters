use super::error::Result;
use crate::{
    app::AppState,
    helper::{hash::generate_hash, jwt::AuthUser},
    models::user::{NewUser, User, UserBody},
    repositories::Db,
};
use axum::{async_trait, Json};
// use uuid::Uuid;

#[async_trait]
pub trait UserRepo {
    async fn create(
        &self,
        state: &AppState,
        Json(req): Json<UserBody<NewUser>>,
    ) -> Result<Json<UserBody<User>>>;
}

pub struct UserRepoImpl {
    pool: Db,
}

impl UserRepoImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn create(
        &self,
        state: &AppState,
        Json(req): Json<UserBody<NewUser>>,
    ) -> Result<Json<UserBody<User>>> {
        let password_hash = generate_hash(&req.user.password)?;

        let user_id = sqlx::query_scalar!(
            r#"INSERT INTO "user" (name, email, passwd_hash) values ($1, $2, $3) returning id "#,
            req.user.name,
            req.user.email,
            password_hash,
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(Json(UserBody {
            user: User {
                email: req.user.email,
                name: req.user.name,
                token: AuthUser { user_id }.to_jwt(&state.secret)?,
                bio: "".to_string(),
                avatar: None,
            },
        }))
    }
}
