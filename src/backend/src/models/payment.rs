use candid::{CandidType, Deserialize};
use ic_ledger_types::{Tokens, AccountIdentifier};

#[derive(CandidType, Deserialize, Clone)]
pub enum PaymentMethod {
    ICP,
    BTC,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Payment {
    pub id: String,
    pub charge_id: String,
    pub method: PaymentMethod,
    pub amount: Tokens,
    pub status: PaymentStatus,
    pub created_at: u64,
    pub transaction_id: Option<String>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum PaymentStatus {
    Pending,
    Confirmed,
    Failed,
    Refunded,
}
