use crate::{
    dto::{
        series::{SeriesRequest, UpdateSeriesRequest},
        Direction, PageQueryParam,
    },
    error::{AppError, AppResult, Resource, ResourceType},
};
use entity::series as SeriesEntity;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, Set,
    TransactionTrait,
};
use std::cmp;

pub async fn create(
    dbc: &DatabaseConnection,
    user_id: i32,
    series_data: &SeriesRequest,
) -> AppResult<i32> {
    let model = SeriesEntity::ActiveModel {
        name: Set(series_data.name.to_owned()),
        description: Set(series_data.description.to_owned()),
        cover: Set(series_data.cover.to_owned().unwrap_or("".to_string())),
        status: Set(series_data.status),
        nums: Set(series_data.nums),
        r#type: Set(series_data.r#type),
        user_id: Set(user_id),
        ..Default::default()
    }
    .insert(dbc)
    .await?;

    Ok(model.id)
}

pub async fn update(
    dbc: &DatabaseConnection,
    series_id: i32,
    update_data: &UpdateSeriesRequest,
) -> AppResult<()> {
    let model = SeriesEntity::Entity::find_by_id(series_id).one(dbc).await?;
    if model.is_none() {
        return Err(AppError::NotFound(Resource {
            r#type: ResourceType::Series,
            detail: "Not found this series".to_string(),
        }));
    }
    let tx = dbc.begin().await?;

    let mut model: SeriesEntity::ActiveModel = model.unwrap().into();

    if let Some(name) = &update_data.name {
        model.name = Set(name.to_string());
    }

    if let Some(description) = &update_data.description {
        model.description = Set(Some(description.to_string()));
    }

    if let Some(cover) = &update_data.cover {
        model.cover = Set(cover.to_string());
    }

    if let Some(status) = update_data.status {
        model.status = Set(status);
    }

    if let Some(nums) = update_data.nums {
        model.nums = Set(nums);
    }

    if let Some(r#type) = update_data.r#type {
        model.r#type = Set(r#type);
    }

    model.update(&tx).await?;
    tx.commit().await?;

    Ok(())
}

pub async fn read_by_id(
    dbc: &DatabaseConnection,
    series_id: i32,
) -> AppResult<Option<SeriesEntity::Model>> {
    let model = SeriesEntity::Entity::find_by_id(series_id).one(dbc).await?;
    Ok(model)
}

pub async fn read_all(
    dbc: &DatabaseConnection,
    param: &PageQueryParam,
) -> AppResult<Vec<SeriesEntity::Model>> {
    let mut select = SeriesEntity::Entity::find();

    match param.order_direction {
        Some(Direction::Desc) => {
            select = select.order_by_desc(SeriesEntity::Column::CreatedAt);
        }
        _ => {
            select = select.order_by_asc(SeriesEntity::Column::CreatedAt);
        }
    }

    let models = select
        .paginate(dbc, cmp::max(param.per_page.unwrap_or(10), 1))
        .fetch_page(cmp::max(param.page.unwrap_or(1) - 1, 0))
        .await?;

    Ok(models)
}

pub async fn delete_by_id(dbc: &DatabaseConnection, series_id: i32) -> AppResult<()> {
    SeriesEntity::Entity::delete_by_id(series_id)
        .exec(dbc)
        .await?;

    Ok(())
}
