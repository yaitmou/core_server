use serde::{Deserialize, Serialize};
/**
 * Instead of having a role as a simple string stored in the profile collections we should store it
 * in its own collection so that we limit the allowed roles!
 */
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    #[default]
    Authenticated,
    Admin,
    SuperUser,
    Other(String),
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }
}
