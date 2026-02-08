use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};

use crate::{
    api::auth::domain::entities::user_role::UserRole,
    core::{rand_token_service::TokenService, AppError, MsgBuilder},
};
const MAX_RESET_PWD_ATTEMPTS: i32 = 5;
const MAX_RESEND_ATTEMPTS: i32 = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub reset_pwd_token: Option<String>,
    pub reset_pwd_count: i32,
    pub activation_token: Option<String>,
    pub activation_count: i32,
    pub is_logged_out: bool,
    pub verified: bool,
    pub banned: bool,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, first_name: String, last_name: String) -> Self {
        let now = Utc::now();
        Self {
            id: "".to_string(), // Will be set when persisting
            email,
            password: "".to_string(),
            first_name,
            last_name,
            role: UserRole::Authenticated,
            verified: false,
            banned: false,
            reset_pwd_token: None,
            reset_pwd_count: 0,
            activation_token: None,
            activation_count: 0,
            is_logged_out: true,
            created_at: now,
        }
    }
    pub fn can_continue(&self) -> Result<(), AppError> {
        /* ································································· [ If User Verified ] */
        if !self.verified {
            let msg = MsgBuilder::custom("Your account has not been verified yet! To activate your account, please follow activation instructions sent to your email address");
            let err = AppError::AccountNotActive(msg);
            return Err(err);
        }

        /* ··································································· [ Is User Banned ] */
        if self.banned {
            let err = AppError::InternalServer(MsgBuilder::try_later());
            return Err(err);
        }

        Ok(())
    }

    pub fn set_reset_pwd_token(&mut self) -> Result<(), AppError> {
        if self.reset_pwd_count >= MAX_RESET_PWD_ATTEMPTS {
            return Err(AppError::Forbidden(MsgBuilder::try_later()));
        }

        self.reset_pwd_token = Some(TokenService::generate_token());
        self.reset_pwd_count += 1;

        Ok(())
    }

    pub fn set_activation_token(&mut self) -> Result<(), AppError> {
        if self.activation_count > MAX_RESEND_ATTEMPTS {
            return Err(AppError::Forbidden(MsgBuilder::try_later()));
        }

        self.activation_token = Some(TokenService::generate_token());
        self.activation_count += 1;

        Ok(())
    }

    pub fn verify_pwd(&self, pwd: String) -> Result<(), AppError> {
        let is_valid = match verify(&pwd, &self.password) {
            Ok(value) => value,
            Err(_) => return Err(AppError::InternalServer(MsgBuilder::try_later())),
        };

        if !is_valid {
            let msg = MsgBuilder::try_again("credentials");
            let err = AppError::AuthenticationFailed(msg);
            return Err(err);
        }

        Ok(())
    }
    pub fn change_pwd(&mut self, new_pwd: String, old_pwd: String) -> Result<(), AppError> {
        self.verify_pwd(old_pwd)?;
        self.set_pwd(new_pwd)?;

        Ok(())
    }

    pub fn set_pwd(&mut self, pwd: String) -> Result<(), AppError> {
        let hashed_pwd = match hash(pwd, DEFAULT_COST) {
            Ok(value) => value,
            Err(_) => {
                return Err(AppError::InternalServer(MsgBuilder::try_later()));
            }
        };
        self.password = hashed_pwd;
        Ok(())
    }

    pub fn re_set_pwd(&mut self, pwd: String, token: String) -> Result<(), AppError> {
        if self.reset_pwd_token != Some(token) {
            let msg = MsgBuilder::try_again("reste pin");
            return Err(AppError::Forbidden(msg));
        }

        self.set_pwd(pwd)?;
        self.reset_pwd_token = None;
        self.reset_pwd_count = 0;
        Ok(())
    }

    pub fn log_out(&mut self) -> () {
        self.is_logged_out = true;
        ()
    }
    pub fn log_in(&mut self) -> () {
        self.is_logged_out = false;
        ()
    }
}
