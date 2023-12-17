use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArticleBody<T = Article> {
    article: T,
}

#[derive(Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub description: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Serialize)]
pub struct Article {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author: User,
}

pub struct ArticleFromQuery {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author_id: Uuid,
    pub author_name: String,
    pub author_email: String,
    pub author_bio: String,
    pub author_avatar: Option<String>,
}

impl ArticleFromQuery {
    pub fn into_article(self) -> Article {
        Article {
            title: self.title,
            slug: self.slug,
            description: self.description,
            body: self.body,
            created_at: self.created_at,
            updated_at: self.updated_at,
            author: User {
                id: self.author_id,
                email: self.author_email,
                name: self.author_name,
                bio: self.author_bio,
                avatar: self.author_avatar,
            },
        }
    }
}
