//! Service Layer Error Handling

use thiserror::Error;

/// Unified error type for service operations
#[derive(Debug, Error)]
pub enum ServiceError {
    /// Database related errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Resource not found errors
    #[error("Not found: {0}")]
    NotFound(String),

    /// Unauthorized access errors
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Conflict errors
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Internal server errors
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias for service operations
pub type ServiceResult<T> = Result<T, ServiceError>;

impl ServiceError {
    /// Create a validation error
    pub fn validation(message: &str) -> Self {
        ServiceError::Validation(message.to_string())
    }

    /// Create a not found error
    pub fn not_found(resource: &str) -> Self {
        ServiceError::NotFound(format!("{} not found", resource))
    }

    /// Create an unauthorized error
    pub fn unauthorized(message: &str) -> Self {
        ServiceError::Unauthorized(message.to_string())
    }

    /// Create a conflict error
    pub fn conflict(message: &str) -> Self {
        ServiceError::Conflict(message.to_string())
    }

    /// Create an internal error
    pub fn internal(message: &str) -> Self {
        ServiceError::Internal(message.to_string())
    }
}