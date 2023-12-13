use std::sync::Arc;

use self::{
    article::{ArticleRepo, ArticleRepoImpl},
    user::{UserRepo, UserRepoImpl},
};

pub mod article;
pub mod store;
pub mod user;

pub use store::{new_db_pool, Db};

pub async fn create_repositories() -> RepoImpls {
    let db_pool = Arc::new(new_db_pool().await);
    RepoImpls::new(
        UserRepoImpl::new(db_pool.clone()),
        ArticleRepoImpl::new(db_pool.clone()),
    )
}

pub struct RepoImpls {
    pub user: UserRepoImpl,
    pub article: ArticleRepoImpl,
}

impl RepoImpls {
    pub fn new(user_repo_impl: UserRepoImpl, article_repo_impl: ArticleRepoImpl) -> Self {
        Self {
            user: user_repo_impl,
            article: article_repo_impl,
        }
    }
}

pub trait Repositories {
    type UserRepoImpl: UserRepo;
    type ArticleRepoImpl: ArticleRepo;
    fn user(&self) -> &Self::UserRepoImpl;
    fn article(&self) -> &Self::ArticleRepoImpl;
}

impl Repositories for RepoImpls {
    type UserRepoImpl = UserRepoImpl;
    type ArticleRepoImpl = ArticleRepoImpl;

    fn user(&self) -> &Self::UserRepoImpl {
        &self.user
    }

    fn article(&self) -> &Self::ArticleRepoImpl {
        &self.article
    }
}
