use super::{
    database::mysql::Db,
    repository::DatabaseRepositoryImpl,
};
use crate::domain::{
    model::users::User,
    repository::users::UsersRepository,
};

pub struct RepositoriesModule {
    users_repository: DatabaseRepositoryImpl<User>,
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let users_repository = DatabaseRepositoryImpl::new(db.clone());

        Self { users_repository }
    }
}

pub trait RepositoriesModuleExt {
    type UsersRepo: UsersRepository;

    fn users_repository(&self) -> &Self::UsersRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UsersRepo = DatabaseRepositoryImpl<User>;

    fn users_repository(&self) -> &Self::UsersRepo {
        &self.users_repository
    }
}