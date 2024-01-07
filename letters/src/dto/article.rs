use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ArticleRequest {
    pub title: String,
    pub slug: Option<String>,
    pub cover: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub password_hash: Option<String>,
    pub source: Option<u8>,
    pub source_url: Option<String>,
    pub topping: Option<u8>,
    pub status: Option<u8>,
    pub category_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub cover: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub password_hash: Option<String>,
    pub source: Option<u8>,
    pub source_url: Option<String>,
    pub topping: Option<u8>,
    pub status: Option<u8>,
    pub category_id: Option<i32>,
}

#[derive(Serialize)]
pub struct PreviewArticleResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub cover: String,
    pub summary: String,
    pub source: Option<u8>,
    pub topping: Option<u8>,
    pub status: Option<u8>,
    pub category_id: i32,
    pub user_id: i32,
}

impl From<entity::article::Model> for PreviewArticleResponse {
    fn from(value: entity::article::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            slug: value.slug,
            cover: value.cover,
            summary: value.summary,
            source: Some(value.source),
            topping: Some(value.topping),
            status: Some(value.status),
            category_id: value.category_id,
            user_id: value.user_id,
        }
    }
}

#[derive(Serialize)]
pub struct ArticleResponse {
    pub title: String,
    pub cover: String,
    pub content: String,
    pub source: Option<u8>,
    pub source_url: Option<String>,
    pub topping: Option<u8>,
    pub status: Option<u8>,
    pub category_id: i32,
    pub user_id: i32,
    // pub category: ArticleCategory,
    // pub author: ArticleUser,
    // pub tags: Vec<ArticleTag>,
}

impl From<entity::article::Model> for ArticleResponse {
    fn from(value: entity::article::Model) -> Self {
        Self {
            title: value.title,
            cover: value.cover,
            content: value.content,
            source: Some(value.source),
            source_url: Some(value.source_url),
            topping: Some(value.topping),
            status: Some(value.status),
            category_id: value.category_id,
            user_id: value.user_id,
        }
    }
}
