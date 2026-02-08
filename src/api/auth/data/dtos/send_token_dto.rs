use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SendTokenDto {
    pub email: String,
}
