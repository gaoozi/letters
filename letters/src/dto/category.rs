use sea_orm::{prelude::DateTimeUtc, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub status: u8,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Serialize, FromQueryResult)]
pub struct ArticleCategory {
    pub id: i32,
    pub name: String,
}

impl From<entity::category::Model> for CategoryResponse {
    fn from(value: entity::category::Model) -> Self {
        Self {
            name: value.name,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
