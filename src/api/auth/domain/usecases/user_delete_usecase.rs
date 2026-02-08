use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::auth::domain::{entities::User, repositories::user_repository::UserRepository},
    core::{AppError, UseCase},
};

pub struct DeleteUser {
    repository: Arc<dyn UserRepository>,
}

impl DeleteUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<String, User> for DeleteUser {
    async fn execute(&self, user_id: String) -> Result<User, AppError> {
        self.repository.delete_one_by_id(&user_id).await
    }
}
