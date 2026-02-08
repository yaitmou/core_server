use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RefreshAccessTokenResponseDto {
    pub id: String,
    pub access_token: String,
    pub refresh_token: String,
}
