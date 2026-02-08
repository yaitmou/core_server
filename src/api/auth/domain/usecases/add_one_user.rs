use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::auth::domain::{entities::User, repositories::user_repository::UserRepository},
    core::{AppError, UseCase},
};

pub struct AddOneUser {
    repository: Arc<dyn UserRepository>,
}

impl AddOneUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<User, User> for AddOneUser {
    async fn execute(&self, user: User) -> Result<User, AppError> {
        // let new_reward = dto.try_into()?;
        self.repository.create_one(&user).await
    }
}
