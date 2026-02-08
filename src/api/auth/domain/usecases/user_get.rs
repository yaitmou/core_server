// Get user by query params.. The get user by id is a special case handled by the get user by id
// usecase

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    api::auth::domain::{entities::User, repositories::user_repository::UserRepository},
    core::{AppError, UseCase},
};

pub struct GetUser {
    repository: Arc<dyn UserRepository>,
}

impl GetUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<HashMap<String, String>, User> for GetUser {
    async fn execute(&self, query: HashMap<String, String>) -> Result<User, AppError> {
        self.repository.find_one(query).await
    }
}
