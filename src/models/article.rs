use super::{category::ArticleCategory, user::Profile};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArticleBody<T = Article> {
    article: T,
}

#[derive(Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub status: i8,
    pub password: Option<String>,
    pub category_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub status: Option<i8>,
    pub read_count: Option<i32>,
    pub like_count: Option<i32>,
    pub is_top: Option<bool>,
    pub password: Option<String>,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Debug)]
pub struct Article {
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub status: i8,
    pub read_count: i32,
    pub like_count: i32,
    pub is_top: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub category: ArticleCategory,
    pub author: Profile,
}

pub struct ArticleFromQuery {
    pub id: i32,
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub password: Option<String>,
    pub author_id: i32,
    pub category_id: i32,
    pub status: i8,
    pub read_count: i32,
    pub like_count: i32,
    pub is_top: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub author_name: Option<String>,
    pub author_avatar: Option<String>,
    pub category_name: Option<String>,
    pub category_description: Option<String>,
    pub tag_names: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PreviewArticle {
    pub title: String,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub read_count: i32,
    pub like_count: i32,
    pub is_top: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub author: Profile,
    pub category: ArticleCategory,
}

impl ArticleFromQuery {
    pub fn into_article(self) -> Article {
        Article {
            title: self.title,
            slug: self.slug,
            content: self.content,
            summary: self.summary,
            cover: self.cover,
            status: self.status,
            read_count: self.read_count,
            like_count: self.like_count,
            is_top: self.is_top,
            created_at: self.created_at,
            updated_at: self.updated_at,
            author: Profile {
                name: self.author_name,
                avatar: self.author_avatar,
            },
            category: ArticleCategory {
                name: self.category_name,
                description: self.category_description,
            },
        }
    }

    pub fn into_preview_article(self) -> PreviewArticle {
        PreviewArticle {
            title: self.title,
            slug: self.slug,
            summary: self.summary,
            cover: self.cover,
            read_count: self.read_count,
            like_count: self.like_count,
            is_top: self.is_top,
            created_at: self.created_at,
            updated_at: self.updated_at,
            author: Profile {
                name: self.author_name,
                avatar: self.author_avatar,
            },
            category: ArticleCategory {
                name: self.category_name,
                description: self.category_description,
            },
        }
    }
}
