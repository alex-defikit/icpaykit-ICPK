use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    #[error("Validation failed: {0}")]
    ValidationError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}
