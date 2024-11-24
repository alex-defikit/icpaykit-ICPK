use crate::errors::request_error::RequestError;
use crate::errors::validation_error::ValidationError;
use serde::de::DeserializeOwned;
use serde_json;

pub fn validate_payload<T: DeserializeOwned>(payload: &str) -> Result<T, RequestError> {
    serde_json::from_str(payload).map_err(|e| {
        RequestError::InvalidPayload(format!("Invalid JSON payload: {}", e))
    })
}

pub fn validate_signature(signature: &str) -> Result<(), RequestError> {
    if signature.len() != 64 {
        return Err(RequestError::InvalidSignature(
            "Signature must be 64 characters long".to_string()
        ));
    }
    
    // Verify hex format
    if !signature.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(RequestError::InvalidSignature(
            "Signature must be hexadecimal".to_string()
        ));
    }
    
    Ok(())
}

pub fn validate_required_fields<T: AsRef<str>>(fields: &[T], data: &serde_json::Value) -> Result<(), ValidationError> {
    for field in fields {
        if !data.get(field.as_ref()).is_some() {
            return Err(ValidationError::RequiredField(
                format!("Missing required field: {}", field.as_ref())
            ));
        }
    }
    Ok(())
}

pub fn validate_webhook_url(url: &str) -> Result<(), ValidationError> {
    if !url.starts_with("https://") {
        return Err(ValidationError::InvalidValue(
            "Webhook URL must use HTTPS".to_string()
        ));
    }
    Ok(())
}
