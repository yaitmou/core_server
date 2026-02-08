use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::auth_token::domain::{
        entities::refresh_token::RefreshToken,
        repositories::refresh_token_repository::RefreshTokenRepository,
    },
    core::{AppError, UseCase},
};

pub struct UpdateOneRefreshToken {
    pub repository: Arc<dyn RefreshTokenRepository>,
}

impl UpdateOneRefreshToken {
    pub fn new(repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<RefreshToken, RefreshToken> for UpdateOneRefreshToken {
    async fn execute(&self, refresh_token: RefreshToken) -> Result<RefreshToken, AppError> {
        self.repository.update_one(&refresh_token).await
    }
}
