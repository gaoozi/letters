use crate::error::Result;
use crate::models::article::{ArticleFromQuery, CreateArticle, PreviewArticle};
use crate::models::{Pag, PagRsp};
use crate::repositories::Db;
use axum::async_trait;
use futures::TryStreamExt;

#[async_trait]
pub trait ArticleRepo {
    async fn create(&self, author_id: i32, article_data: CreateArticle) -> Result<u64>;
    async fn get_by_id(&self, article_id: i32) -> Result<ArticleFromQuery>;
    async fn get_list(&self, pag: &Pag) -> Result<PagRsp<PreviewArticle>>;
    async fn get_list_by_category(
        &self,
        category_name: &str,
        pag: &Pag,
    ) -> Result<PagRsp<PreviewArticle>>;
    async fn get_list_by_tag(&self, tag_name: &str, pag: &Pag) -> Result<PagRsp<PreviewArticle>>;
    async fn update_read_count(&self, article_id: i32) -> Result<()>;
    async fn update_like_count(&self, article_id: i32) -> Result<()>;
    async fn add_article_tag(&self, article_id: i32, tag_id: i32) -> Result<u64>;
    async fn delete_article_tag(&self, article_id: i32, tag_id: i32) -> Result<bool>;
    async fn delete(&self, article_id: i32) -> Result<bool>;
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

    async fn get_by_id(&self, article_id: i32) -> Result<ArticleFromQuery> {
        let article = sqlx::query_as!(
            ArticleFromQuery,
            r#"
                    SELECT
                        a.*,
                        u.name AS author_name,
                        u.avatar AS author_avatar,
                        c.name AS category_name,
                        c.description AS category_description,
                        GROUP_CONCAT(DISTINCT t.name SEPARATOR " ") AS tag_names
                    FROM article a
                    LEFT JOIN user u ON a.author_id = u.id
                    LEFT JOIN category c ON a.category_id = c.id
                    LEFT JOIN article_tag a_t ON a.id = a_t.article_id
                    LEFT JOIN tag t ON a_t.tag_id = t.id
                    GROUP BY a.id
                    HAVING a.id = ?
                "#,
            article_id,
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(article)
    }

    async fn get_list(&self, pag: &Pag) -> Result<PagRsp<PreviewArticle>> {
        let article: Vec<_> = sqlx::query_as!(
            ArticleFromQuery,
            r#"
                SELECT
                        SQL_CALC_FOUND_ROWS a.*,
                        u.name AS author_name,
                        u.avatar AS author_avatar,
                        c.name AS category_name,
                        c.description AS category_description,
                        GROUP_CONCAT(DISTINCT t.name SEPARATOR " ") AS tag_names
                    FROM article a
                    LEFT JOIN user u ON a.author_id = u.id
                    LEFT JOIN category c ON a.category_id = c.id
                    LEFT JOIN article_tag a_t ON a.id = a_t.article_id
                    LEFT JOIN tag t ON a_t.tag_id = t.id
                    GROUP BY a.id
                    ORDER BY a.created_at DESC LIMIT ?, ?;
            "#,
            (pag.page_num.unwrap_or(1) - 1) * pag.page_size.unwrap_or(10),
            pag.page_size.unwrap_or(10),
        )
        .fetch(&*self.pool)
        .map_ok(ArticleFromQuery::into_preview_article)
        .try_collect()
        .await?;

        let total = sqlx::query_scalar!(r#"select FOUND_ROWS() AS total"#)
            .fetch_one(&*self.pool)
            .await?;

        Ok(PagRsp {
            total,
            data: article,
        })
    }

    async fn get_list_by_category(
        &self,
        category_name: &str,
        pag: &Pag,
    ) -> Result<PagRsp<PreviewArticle>> {
        let article: Vec<_> = sqlx::query_as!(
            ArticleFromQuery,
            r#"
                SELECT  SQL_CALC_FOUND_ROWS a.*,
                        u.name AS author_name,
                        u.avatar AS author_avatar,
                        c.name AS category_name,
                        c.description AS category_description,
                        GROUP_CONCAT(DISTINCT t.name SEPARATOR " ") AS tag_names
                    FROM article a
                    LEFT JOIN user u ON a.author_id = u.id
                    LEFT JOIN category c ON a.category_id = c.id
                    LEFT JOIN article_tag a_t ON a.id = a_t.article_id
                    LEFT JOIN tag t ON a_t.tag_id = t.id
                    GROUP BY a.id
                    HAVING a.id = (SELECT id FROM category WHERE name = ?)
                    ORDER BY a.created_at DESC LIMIT ?, ?;
            "#,
            category_name,
            (pag.page_num.unwrap_or(1) - 1) * pag.page_size.unwrap_or(10),
            pag.page_size.unwrap_or(10),
        )
        .fetch(&*self.pool)
        .map_ok(ArticleFromQuery::into_preview_article)
        .try_collect()
        .await?;

        let total = sqlx::query_scalar!(r#"select FOUND_ROWS() AS total"#)
            .fetch_one(&*self.pool)
            .await?;

        Ok(PagRsp {
            total,
            data: article,
        })
    }

    async fn get_list_by_tag(&self, tag_name: &str, pag: &Pag) -> Result<PagRsp<PreviewArticle>> {
        let article: Vec<_> = sqlx::query_as!(
            ArticleFromQuery,
            r#"
                SELECT  SQL_CALC_FOUND_ROWS a.*,
                        u.name AS author_name,
                        u.avatar AS author_avatar,
                        c.name AS category_name,
                        c.description AS category_description,
                        GROUP_CONCAT(DISTINCT t.name SEPARATOR " ") AS tag_names
                    FROM article a
                    LEFT JOIN user u ON a.author_id = u.id
                    LEFT JOIN category c ON a.category_id = c.id
                    LEFT JOIN article_tag a_t ON a.id = a_t.article_id
                    LEFT JOIN tag t ON a_t.tag_id = t.id
                    GROUP BY a.id
                    HAVING a.id = any (SELECT article_id FROM article_tag WHERE tag_id = (SELECT id FROM tag WHERE name = ?))
                    ORDER BY a.created_at DESC LIMIT ?, ?;
            "#,
            tag_name,
            (pag.page_num.unwrap_or(1) - 1) * pag.page_size.unwrap_or(10),
            pag.page_size.unwrap_or(10),
        )
        .fetch(&*self.pool)
        .map_ok(ArticleFromQuery::into_preview_article)
        .try_collect()
        .await?;

        let total = sqlx::query_scalar!(r#"select FOUND_ROWS() AS total"#)
            .fetch_one(&*self.pool)
            .await?;

        Ok(PagRsp {
            total,
            data: article,
        })
    }

    async fn update_read_count(&self, article_id: i32) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE article SET read_count = read_count + 1 WHERE id = ?;
            "#,
            article_id,
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn update_like_count(&self, article_id: i32) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE article SET like_count = like_count + 1 WHERE id = ?;
            "#,
            article_id,
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn add_article_tag(&self, article_id: i32, tag_id: i32) -> Result<u64> {
        let last_id = sqlx::query!(
            r#"
                INSERT INTO article_tag(article_id, tag_id) VALUES (?, ?);
            "#,
            article_id,
            tag_id,
        )
        .execute(&*self.pool)
        .await?
        .last_insert_id();

        Ok(last_id)
    }

    async fn delete_article_tag(&self, article_id: i32, tag_id: i32) -> Result<bool> {
        let rows_effected = sqlx::query!(
            r#"
                DELETE FROM article_tag WHERE article_id = ? AND tag_id = ?;
            "#,
            article_id,
            tag_id,
        )
        .execute(&*self.pool)
        .await?
        .rows_affected();
        Ok(rows_effected >= 1)
    }

    async fn delete(&self, article_id: i32) -> Result<bool> {
        let rows_effected = sqlx::query!(
            r#"
                DELETE FROM article WHERE id = ?;
            "#,
            article_id,
        )
        .execute(&*self.pool)
        .await?
        .rows_affected();
        Ok(rows_effected >= 1)
    }
}
