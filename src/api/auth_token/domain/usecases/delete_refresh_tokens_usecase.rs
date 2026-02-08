use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    api::auth_token::domain::repositories::refresh_token_repository::RefreshTokenRepository,
    core::{AppError, CommandUseCase},
};

pub struct DeleteRefreshTokens {
    repository: Arc<dyn RefreshTokenRepository>,
}

impl DeleteRefreshTokens {
    pub fn new(repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CommandUseCase<String> for DeleteRefreshTokens {
    async fn execute(&self, user_id: String) -> Result<(), AppError> {
        let mut query = HashMap::new();
        query.insert("user_id".to_string(), user_id);
        self.repository.delete_one(query).await?;
        Ok(())
    }
}
