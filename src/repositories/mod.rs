use std::sync::Arc;

use self::user::{UserRepo, UserRepoImpl};

pub mod error;
pub mod user;
pub mod store;

pub use store::{Db, new_db_pool};

pub async fn create_repositories() -> RepoImpls {
    let db_pool = Arc::new(new_db_pool().await);
    RepoImpls::new(
        UserRepoImpl::new(db_pool.clone()),
    )
}

pub struct RepoImpls {
    pub user: UserRepoImpl,
}

impl RepoImpls {
    pub fn new(user_repo_impl: UserRepoImpl) -> Self {
        Self {
            user: user_repo_impl,
        }
    }
}

pub trait Repositories {
    type UserRepoImpl: UserRepo;
    fn user(&self) -> &Self::UserRepoImpl;
}

impl Repositories for RepoImpls {
    type UserRepoImpl = UserRepoImpl;

    fn user(&self) -> &Self::UserRepoImpl {
        &self.user
    }
}
