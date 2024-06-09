use async_trait::async_trait;
use anyhow::Result;

use crate::domain::model::users::{User, UserName};

#[async_trait]
pub trait UsersRepository {
    async fn find(&self, user_name: UserName) -> Result<Option<User>>;
}