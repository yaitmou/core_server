use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VerifyResetPwdPinDto {
    pub email: String,
    pub token: String,
}
