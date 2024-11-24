use crate::models::checkout::{CheckoutCreate, Checkout};
use crate::services::checkout as checkout_service;
use crate::auth::merchant::verify_merchant;
use crate::state::store::STORE;


pub async fn create_checkout(params: CheckoutCreate) -> Result<Checkout, String> {
    let merchant_id = ic_cdk::caller();
    
    if !verify_merchant(&merchant_id) {
        return Err("Unauthorized: Not a registered merchant".to_string());
    }

    Ok(checkout_service::create_checkout(merchant_id, params))
}

pub fn get_checkout(id: String) -> Result<Checkout, String> {
    let merchant_id = ic_cdk::caller();
    
    if !verify_merchant(&merchant_id) {
        return Err("Unauthorized: Not a registered merchant".to_string());
    }

    STORE.with(|store| {
        store.borrow()
            .get_checkout(&id)
            .cloned()
            .ok_or_else(|| "Checkout not found".to_string())
    })
}
