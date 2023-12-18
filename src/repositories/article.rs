use crate::error::Result;
use crate::models::article::CreateArticle;
use crate::repositories::Db;
use axum::async_trait;

#[async_trait]
pub trait ArticleRepo {
    async fn create(&self, author_id: i32, article_data: CreateArticle) -> Result<u64>;
}

pub struct ArticleRepoImpl {
    pool: Db,
}

impl ArticleRepoImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticleRepo for ArticleRepoImpl {
    async fn create(&self, author_id: i32, article_data: CreateArticle) -> Result<u64> {
        let last_id = sqlx::query!(
            r#"
                INSERT INTO article(title, slug, content, summary, cover, status, password, category_id, author_id)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
            "#,
            article_data.title,
            article_data.slug,
            article_data.content,
            article_data.summary,
            article_data.cover,
            article_data.status,
            article_data.password,
            article_data.category_id,
            author_id,
        )
        .execute(&*self.pool)
        .await?
        .last_insert_id();

        Ok(last_id)
    }
}
