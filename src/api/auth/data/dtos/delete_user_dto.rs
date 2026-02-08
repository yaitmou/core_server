use serde::Deserialize;

// Request
#[derive(Debug, Deserialize)]
pub struct DeleteUserDto {
    pub user_id: String,
}
