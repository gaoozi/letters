use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use super::{category::ArticleCategory, user::UserInfo};

#[derive(Deserialize, IntoParams, ToSchema)]
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

#[derive(Deserialize, IntoParams, ToSchema)]
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

#[derive(Serialize, ToSchema)]
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

#[derive(Serialize, ToSchema)]
pub struct ArticleResponse {
    pub title: String,
    pub cover: String,
    pub content: String,
    pub source: Option<u8>,
    pub source_url: Option<String>,
    pub topping: Option<u8>,
    pub status: Option<u8>,
    pub category: ArticleCategory,
    pub author: UserInfo,
    pub tags: Vec<String>,
}

#[derive(Serialize, FromQueryResult, ToSchema)]
pub struct ArticleForQuery {
    pub title: String,
    pub cover: String,
    pub content: String,
    pub source: Option<u8>,
    pub source_url: Option<String>,
    pub topping: Option<u8>,
    pub status: Option<u8>,
    pub author_id: i32,
    pub author_name: String,
    pub category_id: i32,
    pub category_name: String,
    pub tag_names: Option<String>,
}

impl From<ArticleForQuery> for ArticleResponse {
    fn from(value: ArticleForQuery) -> Self {
        Self {
            title: value.title,
            cover: value.cover,
            content: value.content,
            source: value.source,
            source_url: value.source_url,
            topping: value.topping,
            status: value.status,
            author: UserInfo {
                id: value.author_id,
                username: value.author_name,
            },
            category: ArticleCategory {
                id: value.category_id,
                name: value.category_name,
            },
            tags: match value.tag_names {
                Some(names) => names.split(' ').map(|v| v.to_string()).collect(),
                None => Vec::new(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ArticleInfo {
    pub id: i32,
    pub title: String,
}
