use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    api::auth_token::domain::repositories::refresh_token_repository::RefreshTokenRepository,
    core::{AppError, CommandUseCase},
};

pub struct DeleteManyRefreshTokens {
    repository: Arc<dyn RefreshTokenRepository>,
}

impl DeleteManyRefreshTokens {
    pub fn new(repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CommandUseCase<HashMap<String, String>> for DeleteManyRefreshTokens {
    async fn execute(&self, query: HashMap<String, String>) -> Result<(), AppError> {
        self.repository.delete_many(query).await?;
        Ok(())
    }
}
