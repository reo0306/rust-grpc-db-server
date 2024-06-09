use std::sync::Arc;

use crate::adapter::{
    database::mysql::Db,
    module::{RepositoriesModule, RepositoriesModuleExt},
};
use crate::app::usecase::users::UsersUseCase;

pub struct Modules {
    pub users_use_case: UsersUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn users_use_case(&self) -> &UsersUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn users_use_case(&self) -> &UsersUseCase<Self::RepositoriesModule> {
        &self.users_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let users_use_case = UsersUseCase::new(repositories_module.clone());

        Self {
            users_use_case,
        }
    }
}