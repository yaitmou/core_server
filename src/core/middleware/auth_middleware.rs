use std::sync::Arc;

use warp::{filters::header::headers_cloned, http::HeaderMap, reject::Rejection, Filter};

use crate::{api::auth::domain::entities::Claims, core::jwt_service::JwtService};

// Authentication middleware
pub fn auth_middleware(
    jwt_service: Arc<JwtService>,
) -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    headers_cloned().and_then(move |headers: HeaderMap| {
        let jwt_service = jwt_service.clone();
        async move {
            // Read the JWT from the headers ('Authorization')
            match jwt_service.decode_jwt(&headers) {
                Ok(claims) => Ok(claims),
                Err(e) => Err(warp::reject::custom(e)),
            }
            // if the jwt is not present in the auth header
            // we check if we have x-api-key and decode it
        }
    })
}
