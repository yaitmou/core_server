use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::auth::domain::{entities::User, repositories::user_repository::UserRepository},
    core::{AppError, UseCase},
};

pub struct UpdateUser {
    user_repository: Arc<dyn UserRepository>,
}

impl UpdateUser {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UseCase<User, User> for UpdateUser {
    async fn execute(&self, user: User) -> Result<User, AppError> {
        self.user_repository.update_one(&user).await
    }
}
