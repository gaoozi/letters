use crate::{
    dto::tag::{TagRequest, UpdateTagRequest},
    error::{AppError, AppResult, Resource, ResourceType},
};
use entity::tag as TagEntity;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

pub async fn create(dbc: &DatabaseConnection, tag_data: &TagRequest) -> AppResult<i32> {
    let model = TagEntity::ActiveModel {
        name: Set(tag_data.name.clone()),
        description: Set(tag_data.description.clone()),
        r#type: Set(tag_data.r#type),
        status: Set(tag_data.status),
        ..Default::default()
    }
    .insert(dbc)
    .await?;

    Ok(model.id)
}

pub async fn update(
    dbc: &DatabaseConnection,
    tag_id: i32,
    update_data: &UpdateTagRequest,
) -> AppResult<()> {
    let model = read_by_id(dbc, tag_id).await?;
    if model.is_none() {
        return Err(AppError::NotFound(Resource {
            r#type: ResourceType::Tag,
            detail: "Not found this tag".to_string(),
        }));
    }
    let tx = dbc.begin().await?;

    let mut model: TagEntity::ActiveModel = model.unwrap().into();

    if let Some(name) = &update_data.name {
        model.name = Set(name.to_string());
    }

    if let Some(desc) = &update_data.description {
        model.description = Set(Some(desc.to_string()));
    }

    if let Some(r#type) = update_data.r#type {
        model.r#type = Set(r#type);
    }

    if let Some(status) = update_data.status {
        model.status = Set(status);
    }

    model.update(&tx).await?;
    tx.commit().await?;

    Ok(())
}

pub async fn read_by_id(
    dbc: &DatabaseConnection,
    tag_id: i32,
) -> AppResult<Option<TagEntity::Model>> {
    let model = TagEntity::Entity::find_by_id(tag_id).one(dbc).await?;
    Ok(model)
}

pub async fn read_all(dbc: &DatabaseConnection) -> AppResult<Vec<TagEntity::Model>> {
    let models = TagEntity::Entity::find().all(dbc).await?;
    Ok(models)
}

pub async fn check_name_exist(
    dbc: &DatabaseConnection,
    name: &str,
) -> AppResult<Option<TagEntity::Model>> {
    let model = TagEntity::Entity::find()
        .filter(TagEntity::Column::Name.eq(name))
        .one(dbc)
        .await?;

    Ok(model)
}
