use fake::faker::lorem::zh_cn::{Paragraph, Sentence};
use fake::Dummy;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Dummy)]
pub struct SeriesRequest {
    #[dummy(faker = "Sentence(8..32)")]
    pub name: String,
    #[dummy(faker = "Paragraph(5..255)")]
    pub description: Option<String>,
    pub cover: Option<String>,
    pub status: u8,
    pub nums: u32,
    pub r#type: u8,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSeriesRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cover: Option<String>,
    pub status: Option<u8>,
    pub nums: Option<u32>,
    pub r#type: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct SeriesResponse {
    pub name: String,
    pub description: Option<String>,
    pub cover: Option<String>,
    pub status: u8,
    pub nums: u32,
    pub r#type: u8,
    pub published_at: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub user_id: i32,
}

impl From<entity::series::Model> for SeriesResponse {
    fn from(value: entity::series::Model) -> Self {
        Self {
            name: value.name,
            description: value.description,
            cover: Some(value.cover),
            status: value.status,
            nums: value.nums,
            r#type: value.r#type,
            published_at: value.published_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
            user_id: value.user_id,
        }
    }
}
