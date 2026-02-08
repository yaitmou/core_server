use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::core::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisteredEvent {
    pub user_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeletedEvent {
    pub user_id: String,
}

#[async_trait]
pub trait CoreEventHandler: Send + Sync {
    async fn on_user_registered(
        self: Arc<Self>,
        event: &UserRegisteredEvent,
    ) -> Result<(), AppError>;

    async fn on_user_deleted(self: Arc<Self>, event: &UserDeletedEvent) -> Result<(), AppError>;
}

// Optional: A no-op handler for when no handler is needed
pub struct NoopEventHandler;

#[async_trait]
impl CoreEventHandler for NoopEventHandler {
    async fn on_user_registered(
        self: Arc<Self>,
        _event: &UserRegisteredEvent,
    ) -> Result<(), AppError> {
        Ok(())
    }

    async fn on_user_deleted(self: Arc<Self>, _event: &UserDeletedEvent) -> Result<(), AppError> {
        Ok(())
    }
}
