// src/backend/src/services/merchant.rs

// use ic_cdk::export::Principal;
// use ic_ledger_types::AccountIdentifier;
// use crate::state::store::STORE;
// use crate::auth::merchant::verify_merchant;

// pub fn get_account_identifier(merchant_id: &Principal) -> Option<AccountIdentifier> {
//     STORE.with(|store| {
//         store.borrow().get_merchant_account(merchant_id).cloned()
//     })
// }

use candid::Principal;
use ic_ledger_types::{AccountIdentifier, Subaccount};
use crate::state::store::STORE;
use crate::auth::merchant::verify_merchant;

pub fn register_merchant_addresses(
    merchant_id: Principal,
    btc_address: String
) -> Result<(), String> {
    // Verify the merchant is registered
    if !verify_merchant(&merchant_id) {
        return Err("Merchant not registered".to_string());
    }

    // Validate BTC address (basic check)
    if btc_address.is_empty() {
        return Err("Invalid BTC address".to_string());
    }

    // Store merchant addresses
    STORE.with(|store| {
        store.borrow_mut().register_merchant_addresses(
            merchant_id,
            btc_address
        )
    });

    Ok(())
}

pub fn get_merchant_addresses(
    merchant_id: &Principal
) -> Result<(AccountIdentifier, String), String> {
    STORE.with(|store| {
        store.borrow()
            .get_merchant_addresses(merchant_id)
            .map(|addresses| (AccountIdentifier::new(merchant_id, &Subaccount([0; 32])), addresses.btc_address.clone()))
            .ok_or_else(|| "Merchant addresses not found".to_string())
    })
}