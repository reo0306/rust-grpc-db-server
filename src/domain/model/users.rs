use derive_new::new;

#[derive(new)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(new)]
pub struct UserName {
    pub name: String,
}
