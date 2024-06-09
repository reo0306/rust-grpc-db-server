use derive_new::new;

use crate::domain::model::users::UserName;

#[derive(new)]
pub struct FindUserName {
    pub name: String,
}

impl From<FindUserName> for UserName {
    fn from(fun: FindUserName) -> Self {
        UserName {
            name: fun.name
        }
    }
}