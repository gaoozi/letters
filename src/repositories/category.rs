use crate::error::{Error, Result, ResultExt};
use crate::{
    models::category::{Category, CategoryData},
    repositories::Db,
};
use axum::async_trait;

#[async_trait]
pub trait CategoryRepo {
    async fn create(&self, category_data: CategoryData) -> Result<u64>;
    async fn update(&self, category_id: i32, user_data: CategoryData) -> Result<bool>;
    async fn get_all(&self) -> Result<Vec<Category>>;
    async fn get(&self, category_id: i32) -> Result<Category>;
    async fn get_by_name(&self, category_name: &str) -> Result<Category>;
    async fn delete(&self, category_id: i32) -> Result<()>;
}

pub struct CategoryRepoImpl {
    pool: Db,
}

impl CategoryRepoImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepo for CategoryRepoImpl {
    async fn create(&self, category_data: CategoryData) -> Result<u64> {
        let user_id = sqlx::query!(
            r#"
                INSERT INTO category(name, description)
                VALUES (?, ?)
            "#,
            category_data.name,
            category_data.description,
        )
        .execute(&*self.pool)
        .await
        .on_constraint("category_name_key", |_| {
            Error::UnprocessableEntity("category name taken".to_string())
        })?
        .last_insert_id();

        Ok(user_id)
    }

    async fn update(&self, category_id: i32, category_data: CategoryData) -> Result<bool> {
        // returning email, name, bio, avatar
        let effect_rows = sqlx::query!(
            r#"
                update category
                set name = ?,
                    description = ?
                where id = ?
            "#,
            category_data.name,
            category_data.description,
            category_id,
        )
        .execute(&*self.pool)
        .await
        .on_constraint("category_name_key", |_| {
            Error::UnprocessableEntity("category name taken".to_string())
        })?
        .last_insert_id();

        Ok(effect_rows == 1)
    }

    async fn get_all(&self) -> Result<Vec<Category>> {
        let rows = sqlx::query_as!(
            Category,
            r#"
                SELECT id, name, description, created_at, updated_at FROM category
                ORDER BY created_at;
            "#,
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows)
    }

    async fn get(&self, category_id: i32) -> Result<Category> {
        let category = sqlx::query!(
            r#"select  id, name, description, created_at, updated_at from category where id = ?"#,
            category_id,
        )
        .fetch_one(&*self.pool)
        .await
        .on_constraint("category_name_key", |_| {
            Error::UnprocessableEntity("category name taken".to_string())
        })?;

        Ok(Category {
            id: category_id,
            name: category.name,
            description: category.description,
            created_at: Some(category.created_at),
            updated_at: Some(category.updated_at),
        })
    }

    async fn get_by_name(&self, category_name: &str) -> Result<Category> {
        let category = sqlx::query!(
            r#"select  id, name, description, created_at, updated_at from category where name = ?"#,
            category_name,
        )
        .fetch_one(&*self.pool)
        .await
        .on_constraint("category_name_key", |_| {
            Error::UnprocessableEntity("category name taken".to_string())
        })?;

        Ok(Category {
            id: category.id,
            name: category.name,
            description: category.description,
            created_at: Some(category.created_at),
            updated_at: Some(category.updated_at),
        })
    }

    async fn delete(&self, category_id: i32) -> Result<()> {
        sqlx::query!(
            r#"
                delete from category where id = ?
            "#,
            category_id,
        )
        .execute(&*self.pool)
        .await?
        .rows_affected();
        Ok(())
    }
}
