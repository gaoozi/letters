use crate::error::{Error, Result, ResultExt};
use crate::{
    models::article::{Article, ArticleFromQuery, CreateArticle},
    repositories::Db,
};
use axum::async_trait;
use sqlx::types::Uuid;

#[async_trait]
pub trait ArticleRepo {
    async fn create(&self, author_id: Uuid, article_data: CreateArticle) -> Result<Article>;
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
    async fn create(&self, author_id: Uuid, article_data: CreateArticle) -> Result<Article> {
        // generate article slug
        let slug = &article_data.title;

        let article = sqlx::query_as!(
            ArticleFromQuery,
            r#"
                with inserted_article as (
                    INSERT INTO article (author_id, title, slug, description, body) values ($1, $2, $3, $4, $5)
                    returning title, slug, description, body, created_at, updated_at
                )
                select
                    inserted_article.*,
                    id author_id,
                    name author_name,
                    email author_email,
                    bio author_bio,
                    avatar author_avatar
                from inserted_article
                inner join "user" on id = $1
            "#,
            author_id,
            article_data.title,
            slug,
            article_data.description,
            article_data.body,
        )
        .fetch_one(&*self.pool)
        .await
        .on_constraint("article_slug_key", |_| {
            Error::UnprocessableEntity(format!("duplicate article slug: {}", slug))
        })?;

        Ok(article.into_article())
    }
}
