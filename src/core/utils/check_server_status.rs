use std::sync::Arc;

use serde::Serialize;
use warp::{filters::header::headers_cloned, http::HeaderMap, Filter};

use crate::core::{jwt_service::JwtService, response::ApiResponse};

// Information sent by the app to the server...
#[derive(Debug, Serialize)]
pub struct ServerHealthResponseDto {
    is_in_review: bool,
    app_version: String,
    is_jwt_valid: bool,
}

const APPVERSION: &str = "3.4.0+21";
const IN_REVIEW: bool = true;

pub fn check_server_status(
    jwt_service: Arc<JwtService>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("server-status")
        .and(warp::get())
        .and(headers_cloned())
        .map(move |headers: HeaderMap| {
            // Jwt token validity check
            let is_jwt_valid: bool = match jwt_service.decode_jwt(&headers) {
                Ok(_) => true,
                Err(_) => false,
            };

            let server_health_response = ServerHealthResponseDto {
                is_in_review: IN_REVIEW,
                app_version: APPVERSION.to_string(),
                is_jwt_valid: is_jwt_valid,
            };

            let response = ApiResponse::<ServerHealthResponseDto>::success(
                "Server health checked successfully!".to_string(),
                Some(server_health_response),
            );

            warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK)
        })
}
