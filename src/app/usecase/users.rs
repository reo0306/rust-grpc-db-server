use anyhow::Result;
use std::sync::Arc;
use derive_new::new;

use crate::adapter::module::RepositoriesModuleExt;
use crate::app::model::users::FindUserName;
use crate::domain::{
    model::users::User,
    repository::users::UsersRepository,
};

#[derive(new)]
pub struct UsersUseCase<R: RepositoriesModuleExt> {
    repository: Arc<R>,
}

impl <R: RepositoriesModuleExt> UsersUseCase<R> {
    pub async fn find_user(&self, name: FindUserName) -> Result<Option<User>> {
        // FindUserName → UserNameに変換し、パラメータとして渡す
        // UsersTable → Userに変換した型を受け取る
        let user = self.repository
            .users_repository()
            .find(name.into())
            .await?;

        // model/usersでnewして返す？
        match user {
            Some(u) => Ok(Some(u)),
            None => Ok(None),
        }
    }
}