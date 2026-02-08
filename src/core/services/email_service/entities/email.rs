// Copyright (c) 2026 Dr. Younss Ait Mou. All rights reserved.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::core::{EmailAddress, EmailContent};

#[derive(Debug, Clone)]
pub struct Email {
    pub from: Option<EmailAddress>,
    pub to: EmailAddress,
    pub subject: String,
    pub content: EmailContent,
}

impl Email {
    pub fn new(to: EmailAddress, content: EmailContent) -> Self {
        Self {
            // This should be set once per app. in case we want to send email from various app email
            // accounts then we can set this each time. Otherwise it will be pulled from the config
            // env
            from: None,
            to,
            subject: content.subject.clone(),
            content,
        }
    }
}
