use crate::models::{CheckoutCreate, ChargeCreate};
use crate::errors::validation_error::ValidationError;

pub fn validate_checkout(checkout: &CheckoutCreate) -> Result<(), ValidationError> {
    if checkout.name.is_empty() {
        return Err(ValidationError::InvalidField("name cannot be empty".to_string()));
    }
    if checkout.description.is_empty() {
        return Err(ValidationError::InvalidField("description cannot be empty".to_string()));
    }
    if checkout.local_price.amount.is_empty() {
        return Err(ValidationError::InvalidField("price amount cannot be empty".to_string()));
    }
    if checkout.local_price.currency.is_empty() {
        return Err(ValidationError::InvalidField("price currency cannot be empty".to_string()));
    }
    Ok(())
}

pub fn validate_charge(charge: &ChargeCreate) -> Result<(), ValidationError> {
    if charge.name.is_empty() {
        return Err(ValidationError::InvalidField("name cannot be empty".to_string()));
    }
    if charge.description.is_empty() {
        return Err(ValidationError::InvalidField("description cannot be empty".to_string()));
    }
    if charge.local_price.amount.is_empty() {
        return Err(ValidationError::InvalidField("price amount cannot be empty".to_string()));
    }
    if charge.local_price.currency.is_empty() {
        return Err(ValidationError::InvalidField("price currency cannot be empty".to_string()));
    }
    Ok(())
}
