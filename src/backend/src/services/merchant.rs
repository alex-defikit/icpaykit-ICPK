// src/backend/src/services/merchant.rs

// use ic_cdk::export::Principal;
// use ic_ledger_types::AccountIdentifier;
// use crate::state::store::STORE;

// pub fn get_account_identifier(merchant_id: &Principal) -> Option<AccountIdentifier> {
//     STORE.with(|store| {
//         store.borrow().get_merchant_account(merchant_id).cloned()
//     })
// }