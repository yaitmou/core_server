use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::auth::domain::{entities::User, repositories::user_repository::UserRepository},
    core::{
        pagination::{PaginatedParams, PaginatedResponse},
        AppError, UseCase,
    },
};

pub struct GetManyUsers {
    repository: Arc<dyn UserRepository>,
}

impl GetManyUsers {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<PaginatedParams, PaginatedResponse<User>> for GetManyUsers {
    async fn execute(&self, params: PaginatedParams) -> Result<PaginatedResponse<User>, AppError> {
        self.repository.find(params).await
    }
}
