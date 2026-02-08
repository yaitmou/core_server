use serde::Deserialize;

// Request
#[derive(Debug, Deserialize)]
pub struct ChangePwdRDto {
    pub old_pwd: String,
    pub new_pwd: String,
    // This is options and would be used by admin if they have to change a user's password for
    // logged in users the user id is pulled from the claims!
    pub user_id: Option<String>,
}
