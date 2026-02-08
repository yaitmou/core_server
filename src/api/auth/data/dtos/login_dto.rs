use serde::{Deserialize, Serialize};

use crate::api::auth::data::user_response_dto::UserResponseDto;

// Request
#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

// Response
#[derive(Debug, Serialize)]
pub struct LoginResponseDto {
    pub refresh_token: String,
    pub user: UserResponseDto,
}
