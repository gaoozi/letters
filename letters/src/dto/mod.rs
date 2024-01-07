use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub mod article;
pub mod auth;
pub mod category;
pub mod tag;
pub mod user;

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct PageQueryParam {
    pub page: u64,
    pub per_page: u64,
    pub order_by: Option<String>,
    pub order_direction: Option<Direction>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub enum Direction {
    Asc,  // Ascending order
    Desc, // Descending order
}
