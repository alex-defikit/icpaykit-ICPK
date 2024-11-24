use crate::models::charge::{ChargeCreate, Charge};
use crate::services::charge as charge_service;
use crate::auth::merchant::verify_merchant;
// use crate::services::merchant::get_account_identifier;

pub async fn create_charge(params: ChargeCreate) -> Result<Charge, String> {
    let merchant_id = ic_cdk::caller();

    if !verify_merchant(&merchant_id) {
        return Err("Unauthorized: Not a registered merchant".to_string());
    }

    // let merchant_account = get_account_identifier(&merchant_id)
    //     .ok_or_else(|| "Merchant account not found".to_string())?;

    let charge = charge_service::create_charge(merchant_id, params).await;

    Ok(charge)
}

pub fn get_charge(id: String) -> Result<Charge, String> {
    charge_service::get_charge(&id)
        .ok_or_else(|| "Charge not found".to_string())
}

pub fn list_charges() -> Vec<Charge> {
    charge_service::list_charges()
}
