use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use ic_ledger_types::{AccountIdentifier, DEFAULT_SUBACCOUNT, Tokens};
use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, GetUtxosResponse, MillisatoshiPerByte,
};
use models::PaymentMethod;
use std::cell::{Cell, RefCell};

pub mod api;
pub mod models;
pub mod services;
pub mod auth;
pub mod errors;
pub mod utils;
pub mod state;
pub mod bitcoin_wallet;
pub mod bitcoin_api;
pub mod ecdsa_api;
pub mod schnorr_api;

thread_local! {
    // The bitcoin network to connect to.
    //
    // When developing locally this should be `Regtest`.
    // When deploying to the IC this should be `Testnet` or `Mainnet`.
    static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Testnet);

     // The derivation path to use for the threshold key.
    static DERIVATION_PATH: Vec<Vec<u8>> = vec![];

    // The ECDSA key name.
    static KEY_NAME: RefCell<String> = RefCell::new(String::from(""));
}

// Re-export commonly used types
pub use models::{
    checkout::{Checkout, CheckoutCreate},
    charge::{Charge, ChargeCreate},
    webhook::{WebhookConfig, WebhookEvent}
};

pub use errors::{
    auth_error::AuthError,
    request_error::RequestError,
    validation_error::ValidationError
};

pub type Height = u32;
pub type BlockHeader = Vec<u8>;

#[derive(CandidType, Debug, Deserialize, PartialEq, Eq)]
pub struct GetBlockHeadersRequest {
    pub start_height: Height,
    pub end_height: Option<Height>,
    pub network: BitcoinNetwork,
}

/// The response returned for a request for getting the block headers from a given height.
#[derive(CandidType, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct GetBlockHeadersResponse {
    pub tip_height: Height,
    pub block_headers: Vec<BlockHeader>,
}

// Initialize canister state
#[init]
fn init(network: BitcoinNetwork) {
    NETWORK.with(|n| n.set(network));

    KEY_NAME.with(|key_name| {
        key_name.replace(String::from(match network {
            // For local development, we use a special test key with dfx.
            BitcoinNetwork::Regtest => "dfx_test_key",
            // On the IC we're using a test ECDSA key.
            BitcoinNetwork::Mainnet | BitcoinNetwork::Testnet => "test_key_1",
        }))
    });

    state::store::STORE.with(|store| {
        *store.borrow_mut() = state::store::Store::init();
    });
}

// Merchant management
#[update]
fn register_merchant() -> Result<(), String> {
    let caller = ic_cdk::caller();
    auth::merchant::register_merchant(caller)
        .map_err(|e| e.to_string())
}

#[update]
fn register_merchant_addresses(
    btc_address: String
) -> Result<(), String> {
    let caller = ic_cdk::caller();
    services::merchant::register_merchant_addresses(
        caller,
        btc_address
    )
}

// Checkout endpoints
#[update]
async fn create_checkout(params: CheckoutCreate) -> Result<Checkout, String> {
    api::checkout::create_checkout(params).await
}

#[query]
fn get_checkout(id: String) -> Result<Checkout, String> {
    api::checkout::get_checkout(id)
}

// Charge endpoints
#[update]
async fn create_charge(params: ChargeCreate) -> Result<Charge, String> {
    api::charge::create_charge(params).await
}

#[query]
fn get_charge(id: String) -> Result<Charge, String> {
    api::charge::get_charge(id)
}

#[query]
fn list_charges() -> Vec<Charge> {
    api::charge::list_charges()
}

// Webhook endpoints
#[update]
async fn register_webhook(config: WebhookConfig) -> Result<(), String> {
    api::webhook::register_webhook(config).await
}

#[update]
async fn verify_webhook(payload: String, signature: String) -> Result<WebhookEvent, String> {
    api::webhook::verify_webhook(payload, signature).await
}

#[query]
fn get_webhook_config() -> Result<WebhookConfig, String> {
    api::webhook::get_webhook_config()
}

#[update]
async fn process_payment(
    charge_id: String, 
    payment_method: PaymentMethod
) -> Result<u64, String> {
    let caller = ic_cdk::caller();
    let caller_account = AccountIdentifier::new(&caller, &DEFAULT_SUBACCOUNT);
    
    let charge = api::charge::get_charge(charge_id.clone())?;
    
    let amount = Tokens::from_e8s(charge.local_price.amount.parse::<u64>().unwrap());

    services::payment::process_payment(
        caller_account,
        amount,
        charge_id,
        payment_method
    ).await
}

// Bitcoin wallet endpoints

#[update]
pub async fn get_balance(address: String) -> u64 {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_balance(network, address).await
}

/// Returns the UTXOs of the given bitcoin address.
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_utxos(network, address).await
}

/// Returns the block headers in the given height range.
#[update]
pub async fn get_block_headers(start_height: u32, end_height: Option<u32>) -> GetBlockHeadersResponse{
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_block_headers(network, start_height, end_height).await
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn get_current_fee_percentiles() -> Vec<MillisatoshiPerByte> {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_current_fee_percentiles(network).await
}

/// Returns the P2PKH address of this canister at a specific derivation path.
#[update]
pub async  fn get_p2pkh_address() -> String {
    let derivation_path = DERIVATION_PATH.with(|d| d.clone());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());
    bitcoin_wallet::p2pkh::get_address(network, key_name, derivation_path).await
}

/// Sends the given amount of bitcoin from this canister's p2pkh address to the given address.
/// Returns the transaction ID.
#[update]
pub async fn send_from_p2pkh(request: SendRequest) -> String {
    let derivation_path = DERIVATION_PATH.with(|d| d.clone());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2pkh::send(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

/// Returns the P2TR address of this canister at a specific derivation path.
#[update]
pub async fn get_p2tr_script_spend_address() -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(b"script_spend".to_vec());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());

    bitcoin_wallet::p2tr_script_spend::get_address(network, key_name, derivation_path)
        .await
        .to_string()
}

/// Sends the given amount of bitcoin from this canister's p2tr address to the given address.
/// Returns the transaction ID.
#[update]
pub async fn send_from_p2tr_script_spend(request: SendRequest) -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(b"script_spend".to_vec());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2tr_script_spend::send(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

/// Returns the P2TR address of this canister at a specific derivation path.
#[update]
pub async fn get_p2tr_raw_key_spend_address() -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(b"key_spend".to_vec());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());

    bitcoin_wallet::p2tr_raw_key_spend::get_address(network, key_name, derivation_path)
        .await
        .to_string()
}

/// Sends the given amount of bitcoin from this canister's p2tr address to the
/// given address. Returns the transaction ID.
///
/// IMPORTANT: This function uses an untweaked key as the spending key.
///
/// WARNING: This function is not suited for multi-party scenarios where
/// multiple keys are used for spending.
#[update]
pub async fn send_from_p2tr_raw_key_spend(request: SendRequest) -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(b"key_spend".to_vec());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2tr_raw_key_spend::send(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct SendRequest {
    pub destination_address: String,
    pub amount_in_satoshi: u64,
}

// Export Candid interface
candid::export_service!();
