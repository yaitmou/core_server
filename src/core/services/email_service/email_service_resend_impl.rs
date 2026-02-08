// Copyright (c) 2026 Dr. Younss Ait Mou. All rights reserved.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use async_trait::async_trait;
use resend_rs::{types::CreateEmailBaseOptions, Resend};

use crate::core::{
    activate_account_email_template::activation_email_template,
    password_reset_email_template::password_reset_confirmation_email_template,
    reset_pwd_token_sent_template::reset_password_email_template, AppError, Config, Email,
    EmailAddress, EmailService, EmailServiceResult,
};

pub struct EmailServicerResendImpl {
    client: Resend,
    config: Config,
}

impl EmailServicerResendImpl {
    pub fn new(config: &Config) -> Self {
        let client = Resend::new(&config.resend_token);
        Self {
            client,
            config: config.clone(),
        }
    }
}

#[async_trait]
impl EmailService for EmailServicerResendImpl {
    async fn send(&self, email: &Email) -> EmailServiceResult<()> {
        let from = format!("{} <{}>", self.config.app_name, self.config.email_from)
            .trim()
            .to_string();

        let to = format!(
            "{} <{}>",
            email.to.full_name.as_deref().unwrap_or(&email.to.address),
            email.to.address
        )
        .trim()
        .to_string();

        let email = CreateEmailBaseOptions::new(from, [to], email.subject.clone())
            .with_html(&email.content.html_content);

        self.client
            .emails
            .send(email)
            .await
            .map_err(|e| AppError::EmailSendingFailed(e.to_string()))?;

        Ok(())
    }

    async fn send_activation_email(&self, to: &str, token: &str) -> EmailServiceResult<()> {
        let content = activation_email_template(to, token, &self.config.app_name);
        let email_address = EmailAddress::new(to)?;
        let email = Email::new(email_address, content);
        self.send(&email).await?;

        Ok(())
    }
    async fn send_reset_pwd_email(&self, to: &str, token: &str) -> EmailServiceResult<()> {
        let content = reset_password_email_template(to, token, &self.config.app_name);

        let email_address = EmailAddress::new(to)?;
        let email = Email::new(email_address, content);
        self.send(&email).await?;

        Ok(())
    }
    async fn send_pwd_reset_confirmation_email(&self, to: &str) -> EmailServiceResult<()> {
        let content = password_reset_confirmation_email_template(to, &self.config.app_name);

        let email_address = EmailAddress::new(to)?;
        let email = Email::new(email_address, content);
        self.send(&email).await?;

        Ok(())
    }
}
