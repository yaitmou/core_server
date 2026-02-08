use crate::{
    api::auth::domain::entities::Claims,
    core::{AppError, MsgBuilder},
};

/// This should be used for when a route is limited to admin
/// It requires an auth middleware and should be used as fellows
/// ```rust
///  warp::path!("admin-route")
///      .and(auth_middleware(self.sl.jwt_service()))
///      .and_then(admin_middleware)
///      .and_then(move |claims: Claims| {
///          let handler = self.clone();
///          async move { handler.handle(claims).await }
///      })
/// ```
pub async fn admin_middleware(claims: Claims) -> Result<Claims, warp::Rejection> {
    if claims.is_admin() {
        Ok(claims)
    } else {
        let msg = MsgBuilder::no_permission_to("perform this action");
        let err = AppError::Forbidden(msg);
        Err(warp::reject::custom(err))
    }
}
