use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use warp::http::HeaderMap;

use crate::{
    api::auth::domain::entities::{Claims, User},
    core::{AppError, Config},
};

struct AuthorizationData {
    token_type: AuthorizationType,
    token: String,
}

enum AuthorizationType {
    JWT,
    APIKEY,
}

pub struct JwtService {
    config: Config,
}

impl JwtService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate_jwt(&self, user: &User) -> Result<String, AppError> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.config.jwt_expiration))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims::new(
            user.id.to_string(),
            user.role.clone(),
            user.first_name.to_string(),
            user.last_name.to_string(),
            user.email.to_string(),
            expiration as usize,
        );

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        ) {
            Ok(result) => result,
            Err(_) => {
                return Err(AppError::JWTError("Could not create the jwt!".to_string()));
            }
        };

        Ok(token)
    }

    /// Decodes and validates a JWT token
    ///
    /// # Arguments
    /// * `token` - The JWT token string to decode
    ///
    /// # Returns
    /// * `Ok(Claims)` - The decoded claims if the token is valid
    /// * `Err(AppError)` - If the token is invalid or expired
    pub fn decode_jwt(&self, headers: &HeaderMap) -> Result<Claims, AppError> {
        let authorization_data =
            self.get_jwt_from_auth_header(headers)
                .ok_or(AppError::InvalidInput(
                    "An error has occurred. Please log out and back in, then try again."
                        .to_string(),
                ))?;
        // Set up validation requirements
        let mut validation = Validation::default();

        // If we get the api-key, we do not need to check its expiry date.
        // As of now we are not expiring the api_tokens. However this behavior
        // might change in future versions
        validation.validate_exp = match authorization_data.token_type {
            AuthorizationType::APIKEY => false,
            AuthorizationType::JWT => true,
        };

        match decode::<Claims>(
            &authorization_data.token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &validation,
        ) {
            Ok(token_data) => {
                let mut claims = token_data.claims;

                claims.api_key = match authorization_data.token_type {
                    AuthorizationType::APIKEY => Some(authorization_data.token),
                    _ => None,
                };

                Ok(claims)
            }
            Err(e) => match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    // we need this error to be more specific so that at the front end we can
                    // properly handle it!
                    Err(AppError::ExpiredAccessToken)
                }
                _ => Err(AppError::Unauthorized(format!(
                    "Invalid access token! {:?}",
                    e
                ))),
            },
        }
    }

    // A helper function to get the jwt from the Authorization header. This handles only Bearer JWT
    // for the time being!!
    fn get_jwt_from_auth_header(&self, headers: &HeaderMap) -> Option<AuthorizationData> {
        if let Some(jwt_auth) = headers.get("Authorization") {
            if let Ok(jwt_str) = jwt_auth.to_str() {
                if jwt_str.starts_with("Bearer ") {
                    return Some(AuthorizationData {
                        token_type: AuthorizationType::JWT,
                        token: jwt_str[7..].to_string(),
                    });
                } else {
                    return None;
                }
            }
        }

        if let Some(api_key) = headers.get("X-API-KEY") {
            if let Ok(api_key_str) = api_key.to_str() {
                return Some(AuthorizationData {
                    token_type: AuthorizationType::APIKEY,
                    token: api_key_str.to_string(),
                });
            }
        }

        None
    }
}
