use candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_ledger_types::Tokens;
use super::checkout::LocalPrice;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ChargeCreate {
    pub name: String,
    pub description: String,
    pub pricing_type: String,
    pub local_price: LocalPrice,
    pub metadata: Option<Metadata>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Metadata {
    pub customer_id: Option<String>,
    pub customer_name: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Charge {
    pub id: String,
    pub merchant_id: Principal,
    pub name: String,
    pub description: String,
    pub pricing_type: String,
    pub local_price: LocalPrice,
    pub metadata: Option<Metadata>,
    pub payments: Vec<CPayment>,
    pub created_at: u64,
    pub status: ChargeStatus,
    pub payment_block_height: Option<u64>,
    pub release_block_height: Option<u64>,
    pub subaccount: [u8; 32],
    pub btc_address: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CPayment {
    pub transaction_id: String,
    pub amount: Tokens,
    pub from: Principal,
    pub created_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum ChargeStatus {
    Pending,
    Completed,
    Failed,
    Expired,
}
