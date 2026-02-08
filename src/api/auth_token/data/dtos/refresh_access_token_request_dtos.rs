use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RefreshAccessTokenRequestDto {
    pub refresh_token: String,
}
