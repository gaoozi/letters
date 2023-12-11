use crate::error::Result;
use crate::helper::hash::generate_hash;
use crate::models::user::UpdateUser;
use crate::{
    models::user::{NewUser, User},
    repositories::Db,
};
use axum::async_trait;
use sqlx::types::Uuid;

#[async_trait]
pub trait UserRepo {
    async fn create(&self, user_data: NewUser) -> Result<User>;
    async fn update(&self, user_id: Uuid, user_data: UpdateUser) -> Result<User>;
    async fn get(&self, user_id: Uuid) -> Result<User>;
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
    async fn create(&self, user_data: NewUser) -> Result<User> {
        let password_hash = generate_hash(&user_data.password)?;

        let user_id = sqlx::query_scalar!(
            r#"INSERT INTO "user" (name, email, passwd_hash) values ($1, $2, $3) returning id "#,
            user_data.name,
            user_data.email,
            password_hash,
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(User {
            id: user_id,
            email: user_data.email,
            name: user_data.name,
            bio: "".to_string(),
            avatar: None,
        })
    }

    async fn update(&self, user_id: Uuid, user_data: UpdateUser) -> Result<User> {
        let user = sqlx::query!(
            // This is how we do optional updates of fields without needing a separate query for each.
            // language=PostgreSQL
            r#"
                update "user"
                set email = coalesce($1, "user".email),
                    name = coalesce($2, "user".name),
                    bio = coalesce($3, "user".bio),
                    avatar = coalesce($4, "user".avatar)
                where id = $5
                returning email, name, bio, avatar
            "#,
            user_data.email,
            user_data.name,
            user_data.bio,
            user_data.avatar,
            user_id,
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(User {
            id: user_id,
            email: user.email,
            name: user.name,
            bio: user.bio,
            avatar: user.avatar,
        })
    }

    async fn get(&self, user_id: Uuid) -> Result<User> {
        let user = sqlx::query!(
            r#"select email, name, bio, avatar from "user" where id = $1"#,
            user_id,
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(User {
            id: user_id,
            email: user.email,
            name: user.name,
            bio: user.bio,
            avatar: user.avatar,
        })
    }
}
