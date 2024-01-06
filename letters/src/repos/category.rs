use crate::{
    dto::category::{CategoryRequest, UpdateCategoryRequest},
    error::{AppError, AppResult, Resource, ResourceType},
};
use entity::category as CategoryEntity;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

pub async fn create(dbc: &DatabaseConnection, category_data: &CategoryRequest) -> AppResult<i32> {
    let model = CategoryEntity::ActiveModel {
        name: Set(category_data.name.clone()),
        description: Set(category_data.description.clone()),
        status: Set(category_data.status),
        ..Default::default()
    }
    .insert(dbc)
    .await?;

    Ok(model.id)
}

pub async fn update(
    dbc: &DatabaseConnection,
    category_id: i32,
    update_data: &UpdateCategoryRequest,
) -> AppResult<()> {
    let model = read_by_id(dbc, category_id).await?;
    if model.is_none() {
        return Err(AppError::NotFound(Resource {
            r#type: ResourceType::Category,
            detail: "Not found this category".to_string(),
        }));
    }
    let tx = dbc.begin().await?;

    let mut model: CategoryEntity::ActiveModel = model.unwrap().into();

    if let Some(name) = &update_data.name {
        model.name = Set(name.to_string());
    }

    if let Some(desc) = &update_data.description {
        model.description = Set(Some(desc.to_string()));
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
    category_id: i32,
) -> AppResult<Option<CategoryEntity::Model>> {
    let model = CategoryEntity::Entity::find_by_id(category_id)
        .one(dbc)
        .await?;
    Ok(model)
}

pub async fn read_all(dbc: &DatabaseConnection) -> AppResult<Vec<CategoryEntity::Model>> {
    let models = CategoryEntity::Entity::find().all(dbc).await?;
    Ok(models)
}

pub async fn check_name_exist(
    dbc: &DatabaseConnection,
    name: &str,
) -> AppResult<Option<CategoryEntity::Model>> {
    let model = CategoryEntity::Entity::find()
        .filter(CategoryEntity::Column::Name.eq(name))
        .one(dbc)
        .await?;

    Ok(model)
}
