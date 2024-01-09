use fake::faker::lorem::en::Word;
use fake::faker::lorem::zh_cn::Sentence;
use fake::Dummy;
use sea_orm::{prelude::DateTimeUtc, FromQueryResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Dummy)]
pub struct CategoryRequest {
    #[dummy(faker = "Word()")]
    pub name: String,
    #[dummy(faker = "Sentence(10..32)")]
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
