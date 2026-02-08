// Copyright (c) 2026 Dr. Younss Ait Mou. All rights reserved.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#[derive(Debug, Clone)]
pub struct EmailContent {
    pub subject: String,
    pub html_content: String,
}

impl EmailContent {
    pub fn new(subject: String, html: String) -> Self {
        Self {
            subject,
            html_content: html,
        }
    }
}
