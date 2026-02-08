use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    api::auth_token::domain::{
        entities::refresh_token::RefreshToken,
        repositories::refresh_token_repository::RefreshTokenRepository,
    },
    core::{AppError, UseCase},
};

pub struct CreateRefreshToken {
    repository: Arc<dyn RefreshTokenRepository>,
}

impl CreateRefreshToken {
    pub fn new(repository: Arc<dyn RefreshTokenRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<RefreshToken, RefreshToken> for CreateRefreshToken {
    async fn execute(&self, refresh_token: RefreshToken) -> Result<RefreshToken, AppError> {
        /* ································································· [ Delete old token ] */
        // Before creating new token, to prevent useless database consumptions, we delete the
        // previous token if any.
        let mut filter = HashMap::new();
        filter.insert("user_id".to_string(), refresh_token.user_id.to_string());

        if let Err(err) = self.repository.delete_one(filter).await {
            match err {
                // If we don't find a refresh token we don't do anything!
                AppError::NotFound(_) => (),
                _ => {
                    return Err(err);
                }
            }
        }

        /* ························································ [ Save the new RefreshToken ] */
        self.repository.create_one(&refresh_token).await
    }
}
