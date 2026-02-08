use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForgotPwdDto {
    pub email: String,
}
