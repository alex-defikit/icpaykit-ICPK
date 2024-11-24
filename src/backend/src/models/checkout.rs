use candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct LocalPrice {
    pub amount: String,
    pub currency: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CheckoutCreate {
    pub name: String,
    pub description: String,
    pub pricing_type: String,
    pub local_price: LocalPrice,
    pub requested_info: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Checkout {
    pub id: String,
    pub payment_link: String,
    pub merchant_id: Principal,
    pub name: String,
    pub description: String,
    pub pricing_type: String,
    pub local_price: LocalPrice,
    pub requested_info: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub status: CheckoutStatus,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum CheckoutStatus {
    Pending,
    Completed,
    Expired,
}
