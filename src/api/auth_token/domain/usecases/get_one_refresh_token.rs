use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    api::auth_token::domain::{
        entities::refresh_token::RefreshToken,
        repositories::refresh_token_repository::RefreshTokenRepository,
    },
    core::{AppError, UseCase},
};

pub struct GetOneRefreshToken {
    repository: Arc<dyn RefreshTokenRepository>,
}

impl GetOneRefreshToken {
    pub fn new(repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<HashMap<String, String>, RefreshToken> for GetOneRefreshToken {
    async fn execute(&self, query: HashMap<String, String>) -> Result<RefreshToken, AppError> {
        // we might need to return appropriate errors here if needed
        // for instance instead of returning an error we can return a not found when record is not
        // found!
        self.repository.find_one(query).await
    }
}
