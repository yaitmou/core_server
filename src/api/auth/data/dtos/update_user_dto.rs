use serde::Deserialize;

use crate::{
    api::auth::domain::entities::{user_role::UserRole, User},
    core::{AppError, Validators},
};

#[derive(Debug, Deserialize, Default, Clone)]
pub struct UpdateUserDto {
    pub id: String, // mainly used to look for user to update on database
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub verified: Option<bool>,
    pub role: Option<UserRole>,
    pub reset_pwd_token: Option<String>,
    pub reset_pwd_count: Option<i32>,
    pub activation_token: Option<String>,
    pub activation_count: Option<i32>,
    pub banned: Option<bool>,
}

impl UpdateUserDto {
    pub fn apply_to(&self, user: User) -> Result<User, AppError> {
        let mut user = user;

        if let Some(email) = &self.email {
            user.email = Validators::validate_email(&email)?;
        }

        if let Some(first_name) = &self.first_name {
            user.first_name = Validators::validate_text_len(
                first_name.to_string(),
                Some("First Name".to_string()),
                Some(3),
                Some(125),
            )?;
        }

        if let Some(last_name) = &self.last_name {
            user.last_name = Validators::validate_text_len(
                last_name.to_string(),
                Some("Last Name".to_string()),
                Some(3),
                Some(125),
            )?;
        }
        if let Some(role) = self.role.clone() {
            user.role = role;
        }

        Ok(user)
    }

    pub fn apply_non_admin_filter(&mut self) {
        self.verified = None;
        self.role = None;
        self.banned = None;
        self.reset_pwd_token = None;
        self.reset_pwd_count = None;
        self.activation_token = None;
        self.activation_count = None;
    }
}
