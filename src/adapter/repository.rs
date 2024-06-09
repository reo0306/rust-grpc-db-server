use derive_new::new;
use std::marker::PhantomData;

use super::database::mysql::Db;

pub mod users;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _marker: PhantomData<T>,
}