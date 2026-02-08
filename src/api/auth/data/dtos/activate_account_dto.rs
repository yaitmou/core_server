use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ActivateAccountDto {
    pub email: String,
    pub token: String,
}
