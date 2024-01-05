use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::Bio).string())
                    .col(ColumnDef::new(User::Avatar).string())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Category::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Category::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Category::Description).string())
                    .col(
                        ColumnDef::new(Category::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Category::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Category::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tag::Name).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Tag::Type)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Tag::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Tag::Description).string())
                    .col(
                        ColumnDef::new(Tag::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tag::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Tag::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Article::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Article::Title).string().not_null())
                    .col(
                        ColumnDef::new(Article::Slug)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Article::Cover)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Article::Content)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Article::Summary)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Article::PasswordHash).string().not_null())
                    .col(
                        ColumnDef::new(Article::Source)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Article::SourceUrl)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Article::Topping)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Article::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Article::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Article::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Article::DeletedAt).timestamp())
                    .col(ColumnDef::new(Article::UserId).integer().not_null())
                    .col(ColumnDef::new(Article::CategoryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("article-author-id")
                            .from(Article::Table, Article::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("article-category-id")
                            .from(Article::Table, Article::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ArticleTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ArticleTag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ArticleTag::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ArticleTag::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(ArticleTag::DeletedAt).timestamp())
                    .col(ColumnDef::new(ArticleTag::ArticleId).integer().not_null())
                    .col(ColumnDef::new(ArticleTag::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("at-article-id")
                            .from(ArticleTag::Table, ArticleTag::ArticleId)
                            .to(Article::Table, Article::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("at-tag-id")
                            .from(ArticleTag::Table, ArticleTag::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Series::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Series::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Series::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Series::Description).string())
                    .col(
                        ColumnDef::new(Series::Cover)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Series::Status)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Series::Nums)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Series::Type)
                            .tiny_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Series::PublishedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Series::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Series::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Series::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("series-author-id")
                            .from(Series::Table, Series::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SeriesArticle::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SeriesArticle::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SeriesArticle::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SeriesArticle::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(SeriesArticle::SeriesId).integer().not_null())
                    .col(
                        ColumnDef::new(SeriesArticle::ArticleId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("sa-series-id")
                            .from(SeriesArticle::Table, SeriesArticle::SeriesId)
                            .to(Series::Table, Series::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("sa-article-id")
                            .from(SeriesArticle::Table, SeriesArticle::ArticleId)
                            .to(Article::Table, Article::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Comment::Content)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Comment::TopCommentId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Comment::ParentCommentId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Comment::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comment::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Comment::DeletedAt).timestamp())
                    .col(ColumnDef::new(Comment::ArticleId).integer().not_null())
                    .col(ColumnDef::new(Comment::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("comment-article-id")
                            .from(Comment::Table, Comment::ArticleId)
                            .to(Article::Table, Article::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("comment-user-id")
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SeriesArticle::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Series::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ArticleTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    PasswordHash,
    Email,
    Bio,
    Avatar,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    Name,
    Status, // 0 - unpublished, 1 - published
    Description,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
    Type,   // 0 - system, 1 - custom
    Status, // 0 - unpublished, 1 - published
    Description,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id,
    Title,
    Slug,
    Cover,
    Content,
    Summary,
    PasswordHash,
    Source,    // 0 - original, 1 - transport, 2 - translate
    SourceUrl, // original link
    Topping,   // 0 - no top, 1 - top
    Status,    // 0 - unpublished, 1 - published
    UserId,
    CategoryId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum ArticleTag {
    Table,
    Id,
    ArticleId,
    TagId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Series {
    Table,
    Id,
    Name,
    Description,
    Cover,
    Status, // 0 - serialize, 1 - finish
    Nums,
    Type, // 0 - free, 1 - login to read, 2 - charge
    PublishedAt,
    CreatedAt,
    UpdatedAt,
    UserId,
}

#[derive(DeriveIden)]
enum SeriesArticle {
    Table,
    Id,
    SeriesId,
    ArticleId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    Content,
    TopCommentId,
    ParentCommentId,
    ArticleId,
    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
