use ic_cdk::export::Principal;
use crate::state::store::STORE;
use crate::errors::auth_error::AuthError;

// const DEFAULT_SUBACCOUNT: Subaccount = Subaccount([0; 32]);

pub fn register_merchant(merchant_id: Principal) -> Result<(), AuthError> {
    if merchant_id == Principal::anonymous() {
        return Err(AuthError::InvalidPrincipal("Anonymous principal not allowed".to_string()));
    }

    // Check if merchant is already registered
    if STORE.with(|store| store.borrow().is_merchant(&merchant_id)) {
        return Err(AuthError::MerchantAlreadyExists("Merchant already registered".to_string()));
    }

    // Register merchant with basic registration
    STORE.with(|store| {
        store.borrow_mut().register_merchant(merchant_id);
    });
    
    Ok(())
}

pub fn verify_merchant(merchant_id: &Principal) -> bool {
    if *merchant_id == Principal::anonymous() {
        return false;
    }
    
    STORE.with(|store| {
        store.borrow().is_merchant(merchant_id)
    })
}

pub fn unregister_merchant(merchant_id: Principal) -> Result<(), AuthError> {
    STORE.with(|store| {
        if !store.borrow().is_merchant(&merchant_id) {
            return Err(AuthError::MerchantNotFound("Merchant not registered".to_string()));
        }
        store.borrow_mut().unregister_merchant(merchant_id);
        Ok(())
    })
}
