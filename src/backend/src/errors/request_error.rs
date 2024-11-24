use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Invalid payload: {0}")]
    InvalidPayload(String),
    
    #[error("Missing field: {0}")]
    MissingField(String),
    
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
}
