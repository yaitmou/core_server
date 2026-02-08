// Copyright (c) 2026 Dr. Younss Ait Mou. All rights reserved.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::core::{AppError, Validators};

pub type EmailAddressResult<T> = Result<T, AppError>;

#[derive(Debug, Clone)]
pub struct EmailAddress {
    pub address: String,
    pub full_name: Option<String>,
}

impl EmailAddress {
    pub fn new(address: &str) -> EmailAddressResult<Self> {
        let address = Validators::validate_email(address)?;
        Ok(Self {
            address,
            full_name: None,
        })
    }
}
