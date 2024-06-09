use anyhow::Result;
use async_trait::async_trait;

use crate::adapter::model::users::UsersTable;
use crate::domain::{
    model::users::{User, UserName},
    repository::users::UsersRepository,
};

use super::DatabaseRepositoryImpl;

#[async_trait]
impl UsersRepository for DatabaseRepositoryImpl<User> {
    async fn find(&self, user_name: UserName) -> Result<Option<User>> {
        let pool = self.pool.0.clone();

        let users_table = sqlx::query_as::<_, UsersTable>(
            r#"
            SELECT 
                id, name
            FROM
                users
            WHERE
                name = ?
            "#,
        )
        .bind(user_name.name)
        .fetch_one(&*pool)
        .await
        .ok();

        match users_table {
            Some(ut) => {
                Ok(Some(ut.try_into()?))
            }
            None => Ok(None)
        }
    }
}
