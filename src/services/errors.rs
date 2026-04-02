//! Service error types

use thiserror::Error;

/// Service-level errors
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Short link not found: {0}")]
    ShortLinkNotFound(String),
    
    #[error("Config not found: {0}")]
    ConfigNotFound(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ServiceError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ServiceError::ShortLinkNotFound(_) => axum::http::StatusCode::NOT_FOUND,
            ServiceError::ConfigNotFound(_) => axum::http::StatusCode::NOT_FOUND,
            ServiceError::InvalidParameter(_) => axum::http::StatusCode::BAD_REQUEST,
            ServiceError::ServiceUnavailable(_) => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            ServiceError::StorageError(_) | ServiceError::InternalError(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
