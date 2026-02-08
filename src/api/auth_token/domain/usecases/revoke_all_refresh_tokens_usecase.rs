// use std::{collections::HashMap, sync::Arc};

// use async_trait::async_trait;

// use crate::{
//     api::auth::domain::repositories::refresh_token_repository::RefreshTokenRepository,
//     core::{AppError, CommandUseCase},
// };

// pub struct RevokeAllRefreshTokens {
//     repository: Arc<dyn RefreshTokenRepository>,
// }

// impl RevokeAllRefreshTokens {
//     pub fn new(repository: Arc<dyn RefreshTokenRepository>) -> Self {
//         Self { repository }
//     }
// }

// #[async_trait]
// impl CommandUseCase<String> for RevokeAllRefreshTokens {
//     async fn execute(&self, user_id: String) -> Result<(), AppError> {
//         let mut query = HashMap::new();
//         query.insert("user_id".to_string(), user_id.to_string());
//         let mut updates = HashMap::new();
//         updates.insert("set".to_string(), "revoked:true".to_string());

//         self.repository.update_many(query, updates);

//         Ok(())
//     }
// }
