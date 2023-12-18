use crate::error::{Error, Result, ResultExt};
use crate::{
    models::tag::{Tag, TagData},
    repositories::Db,
};
use axum::async_trait;

#[async_trait]
pub trait TagRepo {
    async fn create(&self, tag_data: TagData) -> Result<u64>;
    async fn update(&self, tag_id: i32, user_data: TagData) -> Result<bool>;
    async fn get_all(&self) -> Result<Vec<Tag>>;
    async fn get(&self, tag_id: i32) -> Result<Tag>;
    async fn get_by_name(&self, tag_name: &str) -> Result<Tag>;
    async fn delete(&self, tag_id: i32) -> Result<()>;
}

pub struct TagRepoImpl {
    pool: Db,
}

impl TagRepoImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagRepo for TagRepoImpl {
    async fn create(&self, tag_data: TagData) -> Result<u64> {
        let user_id = sqlx::query!(
            r#"
                INSERT INTO tag(name, description)
                VALUES (?, ?)
            "#,
            tag_data.name,
            tag_data.description,
        )
        .execute(&*self.pool)
        .await
        .on_constraint("tag_name_key", |_| {
            Error::UnprocessableEntity("tag name taken".to_string())
        })?
        .last_insert_id();

        Ok(user_id)
    }

    async fn update(&self, tag_id: i32, tag_data: TagData) -> Result<bool> {
        // returning email, name, bio, avatar
        let effect_rows = sqlx::query!(
            r#"
                update tag
                set name = ?,
                    description = ?
                where id = ?
            "#,
            tag_data.name,
            tag_data.description,
            tag_id,
        )
        .execute(&*self.pool)
        .await
        .on_constraint("tag_name_key", |_| {
            Error::UnprocessableEntity("tag name taken".to_string())
        })?
        .last_insert_id();

        Ok(effect_rows == 1)
    }

    async fn get_all(&self) -> Result<Vec<Tag>> {
        let rows = sqlx::query_as!(
            Tag,
            r#"
                SELECT id, name, description, created_at, updated_at FROM tag
                ORDER BY created_at;
            "#,
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows)
    }

    async fn get(&self, tag_id: i32) -> Result<Tag> {
        let tag = sqlx::query!(
            r#"select  id, name, description, created_at, updated_at from tag where id = ?"#,
            tag_id,
        )
        .fetch_one(&*self.pool)
        .await
        .on_constraint("tag_name_key", |_| {
            Error::UnprocessableEntity("tag name taken".to_string())
        })?;

        Ok(Tag {
            id: tag_id,
            name: tag.name,
            description: tag.description,
            created_at: Some(tag.created_at),
            updated_at: Some(tag.updated_at),
        })
    }

    async fn get_by_name(&self, tag_name: &str) -> Result<Tag> {
        let tag = sqlx::query!(
            r#"select  id, name, description, created_at, updated_at from tag where name = ?"#,
            tag_name,
        )
        .fetch_one(&*self.pool)
        .await
        .on_constraint("tag_name_key", |_| {
            Error::UnprocessableEntity("tag name taken".to_string())
        })?;

        Ok(Tag {
            id: tag.id,
            name: tag.name,
            description: tag.description,
            created_at: Some(tag.created_at),
            updated_at: Some(tag.updated_at),
        })
    }

    async fn delete(&self, tag_id: i32) -> Result<()> {
        sqlx::query!(
            r#"
                delete from tag where id = ?
            "#,
            tag_id,
        )
        .execute(&*self.pool)
        .await?
        .rows_affected();
        Ok(())
    }
}
