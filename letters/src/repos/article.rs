use crate::{
    dto::{
        article::{ArticleRequest, UpdateArticleRequest},
        Direction, PageQueryParam,
    },
    error::{AppError, AppResult, Resource, ResourceType},
};
use entity::article as ArticleEntity;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, Set,
    TransactionTrait,
};
use std::cmp;

pub async fn create(
    dbc: &DatabaseConnection,
    user_id: i32,
    article_data: &ArticleRequest,
) -> AppResult<i32> {
    let model = ArticleEntity::ActiveModel {
        title: Set(article_data.title.to_owned()),
        slug: Set(article_data
            .slug
            .to_owned()
            .unwrap_or(article_data.title.to_owned())),
        cover: Set(article_data.cover.to_owned().unwrap_or("".to_string())),
        content: Set(article_data.content.to_owned()),
        summary: Set(article_data.summary.to_owned().unwrap_or("".to_string())),
        password_hash: Set(article_data
            .password_hash
            .to_owned()
            .unwrap_or("".to_string())),
        source: Set(article_data.source.unwrap_or(0)),
        source_url: Set(article_data.source_url.to_owned().unwrap_or("".to_string())),
        topping: Set(article_data.topping.unwrap_or(0)),
        status: Set(article_data.status.unwrap_or(0)),
        category_id: Set(article_data.category_id.unwrap_or(0)),
        user_id: Set(user_id),
        ..Default::default()
    }
    .insert(dbc)
    .await?;

    Ok(model.id)
}

pub async fn update(
    dbc: &DatabaseConnection,
    article_id: i32,
    update_data: &UpdateArticleRequest,
) -> AppResult<()> {
    let model = read_by_id(dbc, article_id).await?;
    if model.is_none() {
        return Err(AppError::NotFound(Resource {
            r#type: ResourceType::Article,
            detail: "Not found this article".to_string(),
        }));
    }
    let tx = dbc.begin().await?;

    let mut model: ArticleEntity::ActiveModel = model.unwrap().into();

    if let Some(name) = &update_data.title {
        model.title = Set(name.to_string());
    }

    if let Some(slug) = &update_data.slug {
        model.slug = Set(slug.to_string());
    }

    if let Some(cover) = &update_data.cover {
        model.cover = Set(cover.to_string());
    }

    if let Some(content) = &update_data.content {
        model.content = Set(content.to_string());
    }

    if let Some(summary) = &update_data.summary {
        model.summary = Set(summary.to_string());
    }

    if let Some(password_hash) = &update_data.password_hash {
        model.password_hash = Set(password_hash.to_string());
    }

    if let Some(source) = update_data.source {
        model.source = Set(source);
    }

    if let Some(source_url) = &update_data.source_url {
        model.source_url = Set(source_url.to_string());
    }

    if let Some(topping) = update_data.topping {
        model.topping = Set(topping);
    }

    if let Some(status) = update_data.status {
        model.status = Set(status);
    }

    if let Some(category_id) = update_data.category_id {
        model.category_id = Set(category_id);
    }

    model.update(&tx).await?;
    tx.commit().await?;

    Ok(())
}

pub async fn read_by_id(
    dbc: &DatabaseConnection,
    article_id: i32,
) -> AppResult<Option<ArticleEntity::Model>> {
    let model = ArticleEntity::Entity::find_by_id(article_id)
        .one(dbc)
        .await?;

    Ok(model)
}

pub async fn read_all(
    dbc: &DatabaseConnection,
    param: &PageQueryParam,
) -> AppResult<Vec<ArticleEntity::Model>> {
    let mut select = ArticleEntity::Entity::find();

    match param.order_direction {
        Some(Direction::Desc) => {
            select = select.order_by_desc(ArticleEntity::Column::CreatedAt);
        }
        _ => {
            select = select.order_by_asc(ArticleEntity::Column::CreatedAt);
        }
    }

    let models = select
        .paginate(dbc, cmp::max(param.per_page, 1))
        .fetch_page(cmp::max(param.page - 1, 0))
        .await?;

    Ok(models)
}

pub async fn delete_by_id(dbc: &DatabaseConnection, article_id: i32) -> AppResult<()> {
    ArticleEntity::Entity::delete_by_id(article_id)
        .exec(dbc)
        .await?;

    Ok(())
}