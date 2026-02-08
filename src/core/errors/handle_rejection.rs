use crate::core::response::ApiResponse;

use super::AppError;
use warp::{http::StatusCode, reply::Response, Rejection, Reply};

pub async fn handle_app_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, format!("Route is not found"))
    } else if let Some(e) = err.find::<AppError>() {
        match e {
            /* ································································· [ Unauthorized ] */
            AppError::Unauthorized(_) | AppError::ExpiredAccessToken => {
                (StatusCode::UNAUTHORIZED, e.to_string())
            }
            /* ···································································· [ Not Found ] */
            AppError::Forbidden(_)
            | AppError::AccountNotActive(_)
            | AppError::AuthenticationFailed(_) => (StatusCode::FORBIDDEN, e.to_string()),

            AppError::NotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),

            /* ·································································· [ Bad Request ] */
            AppError::BadRequest(_) | AppError::EmptyQuery | AppError::InvalidInput(_) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            /* ········································································ [ Email ] */


            /* ························································ [ Internal Server Error ] */
            AppError::Io(_)
            | AppError::FileTooLarge(_, _)
            | AppError::InvalidFileType(_)
            | AppError::InvalidFolderName(_)
            | AppError::UserDirectoryCreationFailed(_)
            | AppError::FileNotFound(_)
            | AppError::Other(_)
            | AppError::UserDirectoryDeletionFailed(_)
            | AppError::UserDirectoryNotFound(_)
            | AppError::EmailSendingFailed(_)
            | AppError::EmailConfigurationError(_)
            | AppError::EmailInvalidAddress(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unknown_error::Internal Server Error".to_string(),
            ),
        }
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        (StatusCode::BAD_REQUEST, format!("bad_request::{e}"))
    } else if let Some(e) = err.find::<warp::reject::MethodNotAllowed>() {
        (StatusCode::METHOD_NOT_ALLOWED, e.to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "unknown_error::Internal Server Error".to_string(),
        )
    };

    let response = ApiResponse::<()>::error(message);
    Ok(warp::reply::with_status(warp::reply::json(&response), code))
}

/// Early rejection
pub async fn early_err_response(err: AppError) -> Response {
    let rejection = warp::reject::custom(err);
    handle_app_rejection(rejection)
        .await
        .unwrap()
        .into_response()
}
