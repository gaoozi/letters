use crate::error::AppResult;
use entity::user as UserEntity;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

// curd
pub async fn read_by_email(
    dbc: &DatabaseConnection,
    email: &str,
) -> AppResult<Option<UserEntity::Model>> {
    let model = UserEntity::Entity::find()
        .filter(UserEntity::Column::Email.eq(email))
        .one(dbc)
        .await?;

    Ok(model)
}
