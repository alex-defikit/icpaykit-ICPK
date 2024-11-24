use candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct WebhookEvent {
    pub id: String,
    pub api_version: String,
    pub created_at: String,
    pub event_type: EventType,
    pub data: EventData,
    pub resource: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum EventType {
    ChargeCreated,
    ChargePending,
    ChargeConfirmed,
    ChargeFailed,
    ChargeDelayed,
    CheckoutCreated,
    CheckoutCompleted, 
    CheckoutExpired,
    ChargeReleaseFailed,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EventData {
    pub id: String,
    pub code: Option<String>,
    pub name: Option<String>,
    pub metadata: Option<WhMetadata>,
    pub payments: Vec<WhPayment>,
    pub hosted_url: Option<String>,
    pub description: Option<String>,
    pub pricing_type: String,
    pub timeline: Vec<Timeline>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct WhMetadata {
    pub customer_id: Option<String>,
    pub customer_name: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct WhPayment {
    pub transaction_id: String,
    pub status: String,
    pub value: PaymentValue,
    pub block_height: u64,
    pub created_at: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PaymentValue {
    pub local: LocalValue,
    pub crypto: CryptoValue,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct LocalValue {
    pub amount: String,
    pub currency: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CryptoValue {
    pub amount: String,
    pub currency: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Timeline {
    pub time: String,
    pub status: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct WebhookConfig {
    pub url: String,
    pub secret: String,
    pub merchant_id: Principal,
    pub enabled: bool,
}
