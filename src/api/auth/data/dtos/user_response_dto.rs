use serde::{Deserialize, Serialize};

use crate::api::auth::domain::entities::{user_role::UserRole, User};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponseDto {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub verified: bool,
    pub role: UserRole,
    #[serde(default)]
    pub reset_pwd_token: Option<String>,
    pub reset_pwd_count: i32,
    #[serde(default)]
    pub activation_token: Option<String>,
    pub activation_count: i32,
    pub is_logged_out: bool,
    pub banned: bool,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            verified: user.verified,
            role: user.role,
            reset_pwd_token: user.reset_pwd_token,
            reset_pwd_count: user.reset_pwd_count,
            activation_token: user.activation_token,
            activation_count: user.activation_count,
            is_logged_out: user.is_logged_out,
            banned: user.banned,
        }
    }
}
