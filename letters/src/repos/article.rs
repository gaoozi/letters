use crate::{
    dto::{
        article::{ArticleForQuery, ArticleRequest, UpdateArticleRequest},
        Direction, PageQueryParam,
    },
    error::{AppError, AppResult, Resource, ResourceType},
};
use entity::article as ArticleEntity;
use entity::article_tag as ArticleTagEntity;
use entity::category as CategoryEntity;
use entity::series as SeriesEntity;
use entity::series_article as SeriesArticleEntity;
use entity::tag as TagEntity;
use entity::user as UserEntity;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType::LeftJoin,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set, TransactionTrait,
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
    let model = ArticleEntity::Entity::find_by_id(article_id)
        .one(dbc)
        .await?;
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
) -> AppResult<Option<ArticleForQuery>> {
    let model = ArticleEntity::Entity::find()
        .column_as(UserEntity::Column::Id, "author_id")
        .column_as(UserEntity::Column::Username, "author_name")
        .column_as(CategoryEntity::Column::Id, "category_id")
        .column_as(CategoryEntity::Column::Name, "category_name")
        .exprs([Expr::cust(
            r#"GROUP_CONCAT(DISTINCT tag.name SEPARATOR " ") AS tag_names"#,
        )])
        .join(LeftJoin, UserEntity::Relation::Article.def().rev())
        .join(LeftJoin, CategoryEntity::Relation::Article.def().rev())
        .join(LeftJoin, ArticleTagEntity::Relation::Article.def().rev())
        .join(LeftJoin, TagEntity::Relation::ArticleTag.def().rev())
        .group_by(ArticleEntity::Column::Id)
        .having(ArticleEntity::Column::Id.eq(article_id))
        .into_model::<ArticleForQuery>()
        .one(dbc)
        .await?;

    Ok(model)
}

pub async fn _read_by_slug(
    dbc: &DatabaseConnection,
    slug: &str,
) -> AppResult<Option<ArticleEntity::Model>> {
    let model = ArticleEntity::Entity::find()
        .filter(ArticleEntity::Column::Slug.eq(slug))
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
        .paginate(dbc, cmp::max(param.per_page.unwrap_or(10), 1))
        .fetch_page(cmp::max(param.page.unwrap_or(1) - 1, 0))
        .await?;

    Ok(models)
}

pub async fn read_all_by_category(
    dbc: &DatabaseConnection,
    category_id: i32,
    param: &PageQueryParam,
) -> AppResult<Vec<ArticleEntity::Model>> {
    let mut select =
        ArticleEntity::Entity::find().filter(ArticleEntity::Column::CategoryId.eq(category_id));

    match param.order_direction {
        Some(Direction::Desc) => {
            select = select.order_by_desc(ArticleEntity::Column::CreatedAt);
        }
        _ => {
            select = select.order_by_asc(ArticleEntity::Column::CreatedAt);
        }
    }

    let models = select
        .paginate(dbc, cmp::max(param.per_page.unwrap_or(10), 1))
        .fetch_page(cmp::max(param.page.unwrap_or(1) - 1, 0))
        .await?;

    Ok(models)
}

pub async fn read_all_by_tag(
    dbc: &DatabaseConnection,
    tag_id: i32,
    param: &PageQueryParam,
) -> AppResult<Vec<ArticleEntity::Model>> {
    let mut select = ArticleEntity::Entity::find()
        .join(LeftJoin, ArticleTagEntity::Relation::Article.def())
        .join(LeftJoin, TagEntity::Relation::ArticleTag.def().rev())
        .having(TagEntity::Column::Id.eq(tag_id));

    match param.order_direction {
        Some(Direction::Desc) => {
            select = select.order_by_desc(ArticleEntity::Column::CreatedAt);
        }
        _ => {
            select = select.order_by_asc(ArticleEntity::Column::CreatedAt);
        }
    }

    let models = select
        .paginate(dbc, cmp::max(param.per_page.unwrap_or(10), 1))
        .fetch_page(cmp::max(param.page.unwrap_or(1) - 1, 0))
        .await?;

    Ok(models)
}

pub async fn read_all_by_series(
    dbc: &DatabaseConnection,
    series_id: i32,
    param: &PageQueryParam,
) -> AppResult<Vec<ArticleEntity::Model>> {
    let mut select = ArticleEntity::Entity::find()
        .join(LeftJoin, SeriesArticleEntity::Relation::Article.def())
        .join(LeftJoin, SeriesEntity::Relation::SeriesArticle.def().rev())
        .having(SeriesEntity::Column::Id.eq(series_id));

    match param.order_direction {
        Some(Direction::Desc) => {
            select = select.order_by_desc(ArticleEntity::Column::CreatedAt);
        }
        _ => {
            select = select.order_by_asc(ArticleEntity::Column::CreatedAt);
        }
    }

    let models = select
        .paginate(dbc, cmp::max(param.per_page.unwrap_or(10), 1))
        .fetch_page(cmp::max(param.page.unwrap_or(1) - 1, 0))
        .await?;

    Ok(models)
}

pub async fn delete_by_id(dbc: &DatabaseConnection, article_id: i32) -> AppResult<()> {
    ArticleEntity::Entity::delete_by_id(article_id)
        .exec(dbc)
        .await?;

    Ok(())
}
