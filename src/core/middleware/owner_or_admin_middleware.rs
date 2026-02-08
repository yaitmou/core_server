use crate::{
    api::auth::domain::entities::Claims,
    core::{AppError, MsgBuilder},
};

/// Will check if the logged in user is either an Admin or the document owner
/// The document must have the owner id (e.g. user_id)
pub async fn owner_or_admin_middleware(owner_id: String, claims: Claims) -> Result<(), AppError> {
    if claims.is_admin() || claims.user_id == owner_id {
        return Ok(());
    }
    let msg = MsgBuilder::no_permission_to("perform this action. You are not the owner");
    Err(AppError::Forbidden(msg))
}
