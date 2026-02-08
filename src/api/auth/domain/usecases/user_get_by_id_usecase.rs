use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::auth::domain::{entities::User, repositories::user_repository::UserRepository},
    core::{AppError, UseCase},
};

pub struct GetUserById {
    repository: Arc<dyn UserRepository>,
}
impl GetUserById {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<String, User> for GetUserById {
    async fn execute(&self, id: String) -> Result<User, AppError> {
        self.repository.find_one_by_id(&id).await
    }
}
