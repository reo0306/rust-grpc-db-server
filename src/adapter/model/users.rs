use sqlx::FromRow;

use crate::domain::model::users::User;

#[derive(FromRow, Debug)]
pub struct UsersTable {
    pub id: i64,
    pub name: String,
}

impl TryFrom<UsersTable> for User {
    type Error = anyhow::Error;

    fn try_from(ut: UsersTable) -> Result<Self, Self::Error> {
        Ok(
            User::new(ut.id, ut.name)
        )
    }
}
