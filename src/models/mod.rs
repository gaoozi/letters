pub mod article;
pub mod category;
pub mod tag;
pub mod user;

pub struct Pag {
    pub page_num: Option<i32>,
    pub page_size: Option<i32>,
}

pub struct PagRsp<T> {
    pub total: i64,
    pub data: Vec<T>,
}