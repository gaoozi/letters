use crate::{
    dto::{
        user::{NewUser, UpdateUserProfile},
        Direction, PageQueryParam,
    },
    error::{AppError, AppResult, Resource, ResourceType},
    utils::hash::generate_hash,
};
use entity::user as UserEntity;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use std::cmp;

pub async fn create(dbc: &DatabaseConnection, user_data: &NewUser) -> AppResult<i32> {
    let user = UserEntity::ActiveModel {
        username: Set(user_data.username.clone()),
        email: Set(user_data.email.clone()),
        password_hash: Set(generate_hash(&user_data.password)?),
        ..Default::default()
    }
    .insert(dbc)
    .await?;

    Ok(user.id)
}

pub async fn update(
    dbc: &DatabaseConnection,
    user_id: i32,
    update_data: &UpdateUserProfile,
) -> AppResult<()> {
    let user = read_by_id(dbc, user_id).await?;
    if user.is_none() {
        return Err(AppError::NotFound(Resource {
            r#type: ResourceType::User,
            detail: "Not found this user".to_string(),
        }));
    }
    let tx = dbc.begin().await?;

    let mut user: UserEntity::ActiveModel = user.unwrap().into();
    if let Some(username) = &update_data.username {
        user.username = Set(username.to_string());
    }

    if let Some(bio) = &update_data.bio {
        user.bio = Set(Some(bio.to_string()));
    }

    if let Some(avatar) = &update_data.avatar {
        user.avatar = Set(Some(avatar.to_string()));
    }

    user.update(&tx).await?;
    tx.commit().await?;

    Ok(())
}

pub async fn update_password(
    dbc: &DatabaseConnection,
    user_id: i32,
    password: &str,
) -> AppResult<()> {
    let password_hash = generate_hash(password)?;
    UserEntity::Entity::update_many()
        .col_expr(UserEntity::Column::PasswordHash, Expr::value(password_hash))
        .filter(UserEntity::Column::Id.eq(user_id))
        .exec(dbc)
        .await?;

    Ok(())
}

pub async fn read_by_id(
    dbc: &DatabaseConnection,
    user_id: i32,
) -> AppResult<Option<UserEntity::Model>> {
    let model = UserEntity::Entity::find_by_id(user_id).one(dbc).await?;
    Ok(model)
}

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

pub async fn _read_all(
    dbc: &DatabaseConnection,
    param: PageQueryParam,
) -> AppResult<Vec<UserEntity::Model>> {
    let mut select = UserEntity::Entity::find();

    match param.order_direction {
        Some(Direction::Desc) => {
            select = select.order_by_desc(UserEntity::Column::CreatedAt);
        }
        _ => {
            select = select.order_by_asc(UserEntity::Column::CreatedAt);
        }
    }

    let models = select
        .paginate(dbc, cmp::max(param.per_page.unwrap_or(10), 1))
        .fetch_page(cmp::max(param.page.unwrap_or(1) - 1, 0))
        .await?;
    Ok(models)
}

pub async fn _check_email_exist(dbc: &DatabaseConnection, email: &str) -> AppResult<bool> {
    let model = UserEntity::Entity::find()
        .filter(UserEntity::Column::Email.eq(email))
        .one(dbc)
        .await?;

    Ok(model.is_some())
}

pub async fn check_username_exist(dbc: &DatabaseConnection, username: &str) -> AppResult<bool> {
    let model = UserEntity::Entity::find()
        .filter(UserEntity::Column::Username.eq(username))
        .one(dbc)
        .await?;

    Ok(model.is_some())
}
