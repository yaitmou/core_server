use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("not_found::{0}")]
    NotFound(String),

    #[error("invalid_input::{0}")]
    InvalidInput(String),

    #[error("unauthorized::{0}")]
    Unauthorized(String),

    #[error("forbidden::{0}")]
    Forbidden(String),

    #[error("account_not_active::{0}")]
    AccountNotActive(String),

    #[error("expired_access_token::Your session has expired. Please login to continue")]
    ExpiredAccessToken,

    #[error("authentication_failed::{0}")]
    AuthenticationFailed(String),

    #[error("database_error::{0}")]
    DatabaseError(String),

    #[error("internal_server_error::{0}")]
    InternalServer(String),

    #[error("jwt_error::{0}")]
    JWTError(String),

    #[error("empty_query::The search query is required")]
    EmptyQuery,

    #[error("bad_request::{0}")]
    BadRequest(String),

    /* ······································································· [ Storage Errors ] */
    #[error("io_error::{0}")]
    Io(#[from] std::io::Error),

    #[error("file_too_large::{0} bytes exceeds limit of {1} bytes")]
    FileTooLarge(u64, u64),

    #[error("invalid_file_type::Not a valid file type. Allowed types: {0:?}")]
    InvalidFileType(Vec<String>),

    #[error("invalid_folder_name::'{0}'. Must be alphanumeric with underscores, 2-50 chars.")]
    InvalidFolderName(String),

    #[error("user_directory_creation_failed::{0}")]
    UserDirectoryCreationFailed(String),

    #[error("file_not_found::{0}")]
    FileNotFound(String),

    #[error("storage_error::{0}")]
    Other(String),

    #[error("failed_to_delete_user_directory::{0}")]
    UserDirectoryDeletionFailed(String),

    #[error("user_directory_not_found::{0}")]
    UserDirectoryNotFound(String),

    /* ········································································· [ Email Errors ] */
    #[error("email_sending_failed::{0}")]
    EmailSendingFailed(String),

    #[error("invalid_email_address::{0}")]
    EmailInvalidAddress(String),

    #[error("email_configuration_error::{0}")]
    EmailConfigurationError(String),
}

impl Reject for AppError {}
