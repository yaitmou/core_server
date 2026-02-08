use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::auth::{
        data::{datasource::user_datasource::UserDataSource, UserMongoModel},
        domain::{entities::User, repositories::user_repository::UserRepository},
    },
    core::CrudRepositoryImpl,
};

pub struct UserRepositoryImpl {
    datasource: Arc<dyn UserDataSource>,
}

// UserRepository Implementations
impl UserRepositoryImpl {
    // constructor
    pub fn new(datasource: Arc<dyn UserDataSource>) -> Self {
        Self { datasource }
    }
}

#[async_trait]
impl CrudRepositoryImpl<User, UserMongoModel, dyn UserDataSource> for UserRepositoryImpl {
    fn get_datasource(&self) -> Arc<dyn UserDataSource> {
        self.datasource.clone()
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {}
