use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    api::auth::domain::repositories::user_repository::UserRepository,
    core::{AppError, UseCase},
};

pub struct DeleteManyUsers {
    repository: Arc<dyn UserRepository>,
}

impl DeleteManyUsers {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<HashMap<String, String>, u64> for DeleteManyUsers {
    async fn execute(&self, query: HashMap<String, String>) -> Result<u64, AppError> {
        self.repository.delete_many(query).await
    }
}
