use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid principal: {0}")]
    InvalidPrincipal(String),
    
    #[error("Merchant not found: {0}")]
    MerchantNotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
} 