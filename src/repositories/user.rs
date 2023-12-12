use crate::error::{Error, Result, ResultExt};
use crate::helper::hash::{generate_hash, verify_password};
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
    async fn check(&self, email: String, password: String) -> Result<User>;
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
        .await
        .on_constraint("user_name_key", |_| {
            Error::UnprocessableEntity("username taken".to_string())
        })
        .on_constraint("user_email_key", |_| {
            Error::UnprocessableEntity("email taken".to_string())
        })?;

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
        .await
        .on_constraint("user_name_key", |_| {
            Error::UnprocessableEntity("username taken".to_string())
        })
        .on_constraint("user_email_key", |_| {
            Error::UnprocessableEntity("email taken".to_string())
        })?;

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
        .await
        .on_constraint("user_name_key", |_| {
            Error::UnprocessableEntity("username taken".to_string())
        })
        .on_constraint("user_email_key", |_| {
            Error::UnprocessableEntity("email taken".to_string())
        })?;

        Ok(User {
            id: user_id,
            email: user.email,
            name: user.name,
            bio: user.bio,
            avatar: user.avatar,
        })
    }

    async fn check(&self, email: String, password: String) -> Result<User> {
        let user = sqlx::query!(
            r#"select id, email, name, bio, avatar, passwd_hash from "user" where email = $1"#,
            email,
        )
        .fetch_optional(&*self.pool)
        .await?
        .ok_or(Error::UnprocessableEntity(
            "email does not exist".to_string(),
        ))?;

        verify_password(&password, &user.passwd_hash)?;

        Ok(User {
            id: user.id,
            email: user.email,
            name: user.name,
            bio: user.bio,
            avatar: user.avatar,
        })
    }
}
