use serde::Deserialize;

// Request
#[derive(Debug, Deserialize)]
pub struct DeleteManyUsersDto {
    pub ids: Vec<String>,
}
