use fake::faker::lorem::en::Word;
use fake::faker::lorem::zh_cn::Sentence;
use fake::Dummy;
use sea_orm::{prelude::DateTimeUtc, FromQueryResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Dummy)]
pub struct TagRequest {
    #[dummy(faker = "Word()")]
    pub name: String,
    #[dummy(faker = "Sentence(10..32)")]
    pub description: Option<String>,
    pub r#type: u8,
    pub status: u8,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<u8>,
    pub status: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Serialize, FromQueryResult)]
pub struct ArticleTag {
    pub id: i32,
    pub name: String,
}

impl From<entity::tag::Model> for TagResponse {
    fn from(value: entity::tag::Model) -> Self {
        Self {
            name: value.name,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
