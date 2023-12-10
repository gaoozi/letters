use crate::error::{Error, Result};
use crate::models::user::LoginUser;
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

    async fn login(
        &self,
        state: &AppState,
        Json(req): Json<UserBody<LoginUser>>,
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

    async fn login(
        &self,
        state: &AppState,
        Json(req): Json<UserBody<LoginUser>>,
    ) -> Result<Json<UserBody<User>>> {
        let user = sqlx::query!(
            r#"select id, email, name, bio, avatar, passwd_hash from "user" where email = $1"#,
            req.user.email,
        )
        .fetch_optional(&*self.pool)
        .await?
        .ok_or(Error::UnprocessableEntity(
            "Email does not exist".to_string(),
        ))?;

        Ok(Json(UserBody {
            user: User {
                email: user.email,
                token: AuthUser { user_id: user.id }.to_jwt(&state.secret)?,
                name: user.name,
                bio: user.bio,
                avatar: user.avatar,
            },
        }))
    }
}
