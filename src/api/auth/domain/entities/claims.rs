use serde::{Deserialize, Serialize};

use crate::api::auth::domain::entities::user_role::UserRole;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String,
    pub user_role: UserRole,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub exp: usize,
    // The api_key will only be used when api calls are used.
    // It will be used to compare against the stored hash version in the user's record.
    #[serde(default)]
    pub api_key: Option<String>,
}

impl Claims {
    pub fn new(
        user_id: String,
        user_role: UserRole,
        first_name: String,
        last_name: String,
        email: String,
        expiration: usize,
    ) -> Self {
        Self {
            user_id,
            user_role,
            first_name,
            last_name,
            email,
            exp: expiration,
            api_key: None,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.user_role.is_admin()
    }
}
