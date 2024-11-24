use ic_cdk::api::call::call;
use ic_ledger_types::{
    AccountIdentifier, Memo, Tokens, 
    TransferArgs, DEFAULT_FEE, Subaccount, 
    TransferError
};
use candid::{Principal, CandidType, Deserialize};
use lazy_static::lazy_static;

use crate::models::{
    ChargeStatus, 
    payment::{Payment, PaymentMethod, PaymentStatus}
};
use crate::state::store::STORE;
use crate::services::{
    charge::*,
    merchant
};
use crate::utils::generate_id;
use crate::{
    auth, Charge,
    get_balance, send_from_p2pkh, 
    SendRequest
};

const LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const DEFAULT_SUBACCOUNT: Subaccount = Subaccount([0; 32]);

lazy_static! {
    static ref PAYMENT_GATEWAY_PRINCIPAL: Principal = ic_cdk::api::id();
    static ref PAYMENT_GATEWAY_ACCOUNT: AccountIdentifier = 
        AccountIdentifier::new(&PAYMENT_GATEWAY_PRINCIPAL, &DEFAULT_SUBACCOUNT);
}

// External Bitcoin Payment Wrapper
pub async fn process_btc_payment_external(
    charge_id: String,
    bitcoin_sender_address: Option<String>
) -> Result<u64, String> {
    // Generate a default system account identifier
    let default_payer = AccountIdentifier::new(
        &Principal::management_canister(), 
        &Subaccount([0; 32])
    );

    // Get charge details
    let charge = get_charge(&charge_id)
        .ok_or_else(|| "Charge not found".to_string())?;
    
    let amount = Tokens::from_e8s(
        charge.local_price.amount.parse()
            .map_err(|_| "Invalid amount format".to_string())?
    );

    // Optional: Validate Bitcoin sender if provided
    if let Some(sender_address) = bitcoin_sender_address {
        validate_bitcoin_sender(&sender_address, &charge)?;
    }

    // Process payment with default account
    process_payment(
        default_payer,
        amount,
        charge_id,
        PaymentMethod::BTC
    ).await
}

// Main Payment Processing Function
pub async fn process_payment(
    payer: AccountIdentifier,
    amount: Tokens,
    charge_id: String,
    payment_method: PaymentMethod
) -> Result<u64, String> {
    // Retrieve charge details
    let mut charge = get_charge(&charge_id)
        .ok_or_else(|| "Charge not found".to_string())?;

    // Validate payment amount
    validate_payment_amount(&charge, amount)?;

    // Verify merchant exists
    verify_merchant(&charge.merchant_id)?;

    // Process payment based on method
    match payment_method {
        PaymentMethod::ICP => process_icp_payment(payer, amount, &mut charge).await,
        PaymentMethod::BTC => process_btc_payment(payer, amount, &mut charge).await,
    }
}

// Validation Functions
fn validate_payment_amount(charge: &Charge, amount: Tokens) -> Result<(), String> {
    let expected_amount = charge.local_price.amount.parse::<u64>()
        .map_err(|_| "Invalid amount format".to_string())?;
    
    if amount.e8s() != expected_amount {
        return Err("Invalid payment amount".to_string());
    }
    Ok(())
}

fn verify_merchant(merchant_id: &Principal) -> Result<(), String> {
    if !auth::merchant::verify_merchant(merchant_id) {
        return Err("Merchant not registered".to_string());
    }
    Ok(())
}

fn validate_bitcoin_sender(
    sender_address: &str, 
    charge: &Charge
) -> Result<(), String> {
    // Optional: Implement sender validation logic
    // Could check against whitelist, address format, etc.
    Ok(())
}

// ICP Payment Processing
async fn process_icp_payment(
    payer: AccountIdentifier,
    amount: Tokens,
    charge: &mut Charge
) -> Result<u64, String> {
    let transfer_args = TransferArgs {
        memo: Memo(0),
        amount,
        fee: DEFAULT_FEE,
        from_subaccount: None,
        to: *PAYMENT_GATEWAY_ACCOUNT,
        created_at_time: None,
    };

    let block_height = transfer(transfer_args).await?;

    // Update charge status
    charge.status = ChargeStatus::Pending;
    charge.payment_block_height = Some(block_height);
    update_charge(&charge.id, charge.clone())?;

    // Create payment record
    create_payment_record(
        charge, 
        PaymentMethod::ICP, 
        amount, 
        Some(block_height.to_string())
    );

    // Automatically release funds to merchant
    release_to_merchant(charge.id.clone()).await?;

    Ok(block_height)
}

// Bitcoin Payment Processing
async fn process_btc_payment(
    payer: AccountIdentifier,
    amount: Tokens,
    charge: &mut Charge
) -> Result<u64, String> {
    let btc_address = charge.btc_address.clone();
    
    // Check BTC balance
    let balance = get_balance(btc_address.clone()).await;
    
    if balance < amount.e8s() {
        return Err("Insufficient BTC balance".to_string());
    }

    // Update charge status
    charge.status = ChargeStatus::Pending;
    update_charge(&charge.id, charge.clone())?;

    // Create payment record
    let payment = create_payment_record(
        charge, 
        PaymentMethod::BTC, 
        amount, 
        None
    );

    // Trigger BTC transfer
    let tx_id = send_from_p2pkh(SendRequest {
        destination_address: btc_address,
        amount_in_satoshi: amount.e8s(),
    });

    // Release funds to merchant
    release_to_merchant(charge.id.clone()).await?;

    Ok(0) // Placeholder block height for BTC
}

// Transfer helper function
async fn transfer(args: TransferArgs) -> Result<u64, String> {
    let (result,): (Result<u64, TransferError>,) = call(
        Principal::from_text(LEDGER_CANISTER_ID).unwrap(),
        "transfer",
        (args,),
    )
    .await
    .map_err(|e| format!("Transfer failed: {:?}", e))?;

    result.map_err(|e| format!("Transfer failed: {:?}", e))
}

async fn release_to_merchant(charge_id: String) -> Result<u64, String> {
    let charge = get_charge(&charge_id)
        .ok_or_else(|| "Charge not found".to_string())?;

    // Retrieve merchant addresses
    let (merchant_account, _) = merchant::get_merchant_addresses(&charge.merchant_id)?;

    // Transfer funds to merchant
    let transfer_args = TransferArgs {
        memo: Memo(0),
        amount: Tokens::from_e8s(charge.local_price.amount.parse::<u64>().unwrap()),
        fee: DEFAULT_FEE,
        from_subaccount: None,
        to: merchant_account,
        created_at_time: None,
    };

    let block_height = transfer(transfer_args).await?;

    // Update charge status
    let mut updated_charge = charge.clone();
    updated_charge.status = ChargeStatus::Completed;
    updated_charge.release_block_height = Some(block_height);
    update_charge(&charge_id, updated_charge)?;

    Ok(block_height)
}

fn create_payment_record(
    charge: &Charge, 
    method: PaymentMethod, 
    amount: Tokens,
    transaction_id: Option<String>
) -> Payment {
    let payment = Payment {
        id: generate_id(),
        charge_id: charge.id.clone(),
        method,
        amount,
        status: PaymentStatus::Confirmed,
        created_at: ic_cdk::api::time(),
        transaction_id,
    };

    // Store payment record
    STORE.with(|store| {
        store.borrow_mut().create_payment(payment.clone());
    });

    payment
}