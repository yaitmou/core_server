use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

use crate::api::auth::domain::entities::user_role::UserRole;

#[derive(Debug, Clone, Serialize)]
pub struct RefreshToken {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub user_role: UserRole,
    pub expires_at: DateTime<Utc>,
    pub revoked: bool,

    // Audit fields
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
}

impl RefreshToken {
    pub fn new(
        user_id: String,
        token: String,
        user_role: UserRole,
        expires_in_days: Option<i64>,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::days(expires_in_days.unwrap_or(15));

        Self {
            id: "".to_string(),
            user_id: user_id.clone(),
            token,
            user_role,
            expires_at: expires_at,
            revoked: false,

            // Audit fields
            created_by: user_id.clone(),
            created_at: now,
            updated_by: user_id,
            updated_at: now,
        }
    }

    pub fn set_token(&mut self, new_token: String) -> () {
        self.token = new_token;
        self.expires_at = Utc::now() + Duration::days(15);
    }

    pub fn should_renew(&self) -> bool {
        let expiration_threshold = Duration::hours(24);
        (self.expires_at - Utc::now()) < expiration_threshold
    }
}
