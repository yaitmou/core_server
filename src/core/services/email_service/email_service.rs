// Copyright (c) 2026 Dr. Younss Ait Mou. All rights reserved.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use async_trait::async_trait;

use crate::core::{AppError, Email};

pub type EmailServiceResult<T> = Result<T, AppError>;

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send(&self, email: &Email) -> EmailServiceResult<()>;
    async fn send_activation_email(&self, to: &str, token: &str) -> EmailServiceResult<()>;
    async fn send_reset_pwd_email(&self, to: &str, token: &str) -> EmailServiceResult<()>;
    async fn send_pwd_reset_confirmation_email(&self, to: &str) -> EmailServiceResult<()>;
}
