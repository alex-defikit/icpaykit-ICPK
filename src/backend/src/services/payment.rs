use ic_cdk::api::call::call;
use ic_ledger_types::{
  AccountIdentifier, Memo,
  Tokens, TransferArgs, DEFAULT_FEE, Subaccount, TransferError
};
use candid::Principal;
use lazy_static::lazy_static;

use crate::models::ChargeStatus;
use crate::state::store::STORE;
use crate::services::charge::*;

const LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

enum PaymentMethod {
    ICP,
    BTC,
}

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

//fake Principal

lazy_static! {
    static ref PAYMENT_GATEWAY_PRINCIPAL: Principal = ic_cdk::api::id();
}
// const PAYMENT_GATEWAY_PRINCIPAL: Principal = ic_cdk::api::id();

const DEFAULT_SUBACCOUNT: Subaccount = Subaccount([0; 32]);

lazy_static! {
  static ref PAYMENT_GATEWAY_ACCOUNT: AccountIdentifier = 
      AccountIdentifier::new(&PAYMENT_GATEWAY_PRINCIPAL, &DEFAULT_SUBACCOUNT);
}

pub async fn process_payment(
    from: AccountIdentifier,
    amount: Tokens,
    charge_id: String,
    // payment_method: PaymentMethod,
) -> Result<u64, String> {
    // match payment_method {
    //     PaymentMethod::ICP => {

    //     }
    // }
    // Get charge info first
    let charge = get_charge(&charge_id).unwrap();

    let unique_subaccount = STORE.with(|store| {
        store.borrow().get_transaction_subaccount(&charge_id)
    });
    let unique_account = AccountIdentifier::new(&PAYMENT_GATEWAY_PRINCIPAL, &Subaccount(unique_subaccount.unwrap()));
    // Transfer từ user đến payment gateway
    let transfer_args = TransferArgs {
        memo: Memo(0),
        amount,
        fee: DEFAULT_FEE,
        from_subaccount: None,
        to: unique_account,
        created_at_time: None,
    };
    
    match transfer(transfer_args).await {
        Ok(block_height) => {
            update_charge_status(&charge_id, ChargeStatus::Pending)?;

            let mut charge_clone = charge.clone();
            charge_clone.status = ChargeStatus::Pending;
            charge_clone.payment_block_height = Some(block_height);
            update_charge(&charge_id, charge_clone)?;

            let merchant_account = AccountIdentifier::new(&charge.merchant_id, &DEFAULT_SUBACCOUNT);
            match release_to_merchant(charge_id.clone(), merchant_account).await {
                Ok(_) => Ok(block_height),
                Err(e) => Err(format!("Payment processed but release failed: {}", e)),
            }
        },
        Err(e) => Err(format!("Payment failed: {}", e)),
    }
}

pub async fn release_to_merchant(
    charge_id: String, 
    merchant_account: AccountIdentifier
) -> Result<u64, String> {
    let charge = get_charge(&charge_id).unwrap();
    
    // Verify charge status
    if charge.status != ChargeStatus::Pending {
        return Err("Charge not paid".to_string());
    }

    // Transfer từ gateway đến merchant
    let transfer_args = TransferArgs {
        memo: Memo(0),
        amount: Tokens::from_e8s(charge.local_price.amount.parse::<u64>().unwrap()),
        fee: DEFAULT_FEE,
        from_subaccount: None,
        to: merchant_account,
        created_at_time: None,
    };

    let block_height = transfer(transfer_args).await?;

    let mut charge = get_charge(&charge_id).unwrap();
    charge.status = ChargeStatus::Completed;
    charge.release_block_height = Some(block_height);
    update_charge(&charge_id, charge).unwrap();
    
    Ok(block_height)
}