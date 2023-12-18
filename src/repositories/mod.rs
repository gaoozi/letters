use std::sync::Arc;

use self::{
    article::{ArticleRepo, ArticleRepoImpl},
    user::{UserRepo, UserRepoImpl}, tag::{TagRepoImpl, TagRepo}, category::{CategoryRepoImpl, CategoryRepo},
};

pub mod article;
pub mod category;
pub mod store;
pub mod tag;
pub mod user;

pub use store::{new_db_pool, Db};

pub async fn create_repositories() -> RepoImpls {
    let db_pool = Arc::new(new_db_pool().await);
    RepoImpls::new(
        UserRepoImpl::new(db_pool.clone()),
        ArticleRepoImpl::new(db_pool.clone()),
        TagRepoImpl::new(db_pool.clone()),
        CategoryRepoImpl::new(db_pool.clone())
    )
}

pub struct RepoImpls {
    pub user: UserRepoImpl,
    pub article: ArticleRepoImpl,
    pub tag: TagRepoImpl,
    pub category: CategoryRepoImpl,
}

impl RepoImpls {
    pub fn new(user_repo_impl: UserRepoImpl, article_repo_impl: ArticleRepoImpl, 
        tag_repo_impl: TagRepoImpl, category_repo_impl: CategoryRepoImpl) -> Self {
        Self {
            user: user_repo_impl,
            article: article_repo_impl,
            tag: tag_repo_impl,
            category: category_repo_impl,
        }
    }
}

pub trait Repositories {
    type UserRepoImpl: UserRepo;
    type ArticleRepoImpl: ArticleRepo;
    type TagRepoImpl: TagRepo;
    type CategoryRepoImpl: CategoryRepo;
    fn user(&self) -> &Self::UserRepoImpl;
    fn article(&self) -> &Self::ArticleRepoImpl;
    fn tag(&self) -> &Self::TagRepoImpl;
    fn category(&self) -> &Self::CategoryRepoImpl;
}

impl Repositories for RepoImpls {
    type UserRepoImpl = UserRepoImpl;
    type ArticleRepoImpl = ArticleRepoImpl;
    type TagRepoImpl = TagRepoImpl;
    type CategoryRepoImpl = CategoryRepoImpl;

    fn user(&self) -> &Self::UserRepoImpl {
        &self.user
    }

    fn article(&self) -> &Self::ArticleRepoImpl {
        &self.article
    }

    fn tag(&self) -> &Self::TagRepoImpl {
        &self.tag
    }

    fn category(&self) -> &Self::CategoryRepoImpl {
        &self.category
    }
}
