// use std::{collections::HashMap, sync::Arc};

// use async_trait::async_trait;

// use crate::{
//     api::auth::domain::repositories::refresh_token_repository::RefreshTokenRepository,
//     core::{AppError, CommandUseCase},
// };

// pub struct RevokeRefreshTokensParams {
//     pub token_id: String,
// }

// pub struct RevokeRefreshTokens {
//     repository: Arc<dyn RefreshTokenRepository>,
// }

// impl RevokeRefreshTokens {
//     pub fn new(repository: Arc<dyn RefreshTokenRepository>) -> Self {
//         Self { repository }
//     }
// }

// #[async_trait]
// impl CommandUseCase<RevokeRefreshTokensParams> for RevokeRefreshTokens {
//     async fn execute(&self, params: RevokeRefreshTokensParams) -> Result<(), AppError> {
//         let mut query = HashMap::new();
//         query.insert("token".to_string(), params.token_id.to_string());
//         let mut updates = HashMap::new();
//         updates.insert("set".to_string(), "revoked:true".to_string());

//         self.repository.update_many(query, updates).await?;
//         Ok(())
//     }
// }
