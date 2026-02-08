use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
