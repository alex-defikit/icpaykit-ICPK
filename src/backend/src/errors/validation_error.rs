use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid field: {0}")]
    InvalidField(String),
    
    #[error("Required field missing: {0}")]
    RequiredField(String),
    
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid value: {0}")]
    InvalidValue(String),
}
