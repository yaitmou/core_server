use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResetPasswordDto {
    pub email: String,
    pub token: String,
    pub new_password: String,
}
