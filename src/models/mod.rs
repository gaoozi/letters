use serde::{Deserialize, Serialize};

pub mod article;
pub mod auth;
pub mod category;
pub mod tag;
pub mod user;

#[derive(Debug, Deserialize)]
pub struct Pag {
    pub page_num: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PagRsp<T> {
    pub total: i64,
    pub data: Vec<T>,
}
