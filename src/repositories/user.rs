use crate::error::{Error, Result, ResultExt};
use crate::helper::hash::{generate_hash, verify_password};
use crate::models::user::UpdateUser;
use crate::{
    models::user::{NewUser, User},
    repositories::Db,
};
use axum::async_trait;

#[async_trait]
pub trait UserRepo {
    async fn create(&self, user_data: NewUser) -> Result<u64>;
    async fn update(&self, user_id: i32, user_data: UpdateUser) -> Result<bool>;
    async fn get(&self, user_id: i32) -> Result<User>;
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
    async fn create(&self, user_data: NewUser) -> Result<u64> {
        let password_hash = generate_hash(&user_data.password)?;

        let user_id = sqlx::query!(
            r#"
                INSERT INTO user(name, email, password_hash)
                VALUES (?, ?, ?)
            "#,
            user_data.name,
            user_data.email,
            password_hash,
        )
        .execute(&*self.pool)
        .await
        .on_constraint("user_name_key", |_| {
            Error::UnprocessableEntity("username taken".to_string())
        })
        .on_constraint("user_email_key", |_| {
            Error::UnprocessableEntity("email taken".to_string())
        })?
        .last_insert_id();

        Ok(user_id)
    }

    async fn update(&self, user_id: i32, user_data: UpdateUser) -> Result<bool> {
        // returning email, name, bio, avatar
        let effect_rows = sqlx::query!(
            r#"
                update user
                set email = coalesce(?, user.email),
                    name = coalesce(?, user.name),
                    bio = coalesce(?, user.bio),
                    avatar = coalesce(?, user.avatar)
                where id = ?
            "#,
            user_data.email,
            user_data.name,
            user_data.bio,
            user_data.avatar,
            user_id,
        )
        .execute(&*self.pool)
        .await
        .on_constraint("user_name_key", |_| {
            Error::UnprocessableEntity("username taken".to_string())
        })
        .on_constraint("user_email_key", |_| {
            Error::UnprocessableEntity("email taken".to_string())
        })?
        .last_insert_id();

        Ok(effect_rows == 1)
    }

    async fn get(&self, user_id: i32) -> Result<User> {
        let user = sqlx::query!(
            r#"select id, name, email, bio, avatar, created_at, last_seen, is_active from user where id = ?"#,
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
            created_at: Some(user.created_at),
            last_seen: Some(user.last_seen),
            is_active: Some(user.is_active == 1),
        })
    }

    async fn check(&self, email: String, password: String) -> Result<User> {
        let user = sqlx::query!(
            r#"select id, email, name, bio, avatar, password_hash, created_at, last_seen, is_active from user where email = ?"#,
            email,
        )
        .fetch_optional(&*self.pool)
        .await?
        .ok_or(Error::UnprocessableEntity(
            "email does not exist".to_string(),
        ))?;

        verify_password(&password, &user.password_hash)?;

        Ok(User {
            id: user.id,
            email: user.email,
            name: user.name,
            bio: user.bio,
            avatar: user.avatar,
            created_at: Some(user.created_at),
            last_seen: Some(user.last_seen),
            is_active: Some(user.is_active == 1),
        })
    }
}
