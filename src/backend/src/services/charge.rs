use ic_cdk::api::time;
use ic_cdk::export::Principal;
use crate::utils::id::generate_id;

use crate::models::charge::{Charge, ChargeCreate, ChargeStatus};
use crate::state::store::STORE;
use crate::get_p2pkh_address;
use blake2::{Blake2b, Digest};

fn generate_unique_subaccount(charge_id: &str) -> [u8; 32] {
    let mut hasher = Blake2b::new();
    hasher.update(charge_id.as_bytes());
    let result = hasher.finalize();
    let mut subaccount = [0u8; 32];
    subaccount.copy_from_slice(&result[..32]);
    subaccount
}

pub async fn create_charge(merchant_id: Principal, params: ChargeCreate) -> Charge {
    let charge_id = generate_id();
    let subaccount = generate_unique_subaccount(&charge_id);
    let btc_address: String = get_p2pkh_address().await;
    
    let charge = Charge {
        id: charge_id.clone(),
        merchant_id,
        name: params.name,
        description: params.description,
        pricing_type: params.pricing_type,
        local_price: params.local_price,
        metadata: params.metadata,
        payments: Vec::new(),
        created_at: time(),
        status: ChargeStatus::Pending,
        payment_block_height: None,
        release_block_height: None,
        subaccount: subaccount,
        btc_address: btc_address,
    };

    STORE.with(|store| {
        store.borrow_mut().create_charge(charge.clone());
        store.borrow_mut().register_transaction_subaccount(charge_id, subaccount);
    });

    charge
}

pub fn get_charge(id: &str) -> Option<Charge> {
    STORE.with(|store| {
        store.borrow().get_charge(id).cloned()
    })
}

pub fn list_charges() -> Vec<Charge> {
    STORE.with(|store| {
        store.borrow().list_charges().into_iter().cloned().collect()
    })
}

pub fn update_charge(id: &str, charge: Charge) -> Result<(), String> {
  STORE.with(|store| {
      let mut store = store.borrow_mut();
      if store.get_charge(id).is_none() {
          return Err("Charge not found".to_string());
      }
      store.update_charge(id.to_string(), charge);
      Ok(())
  })
}

pub fn update_charge_status(id: &str, status: ChargeStatus) -> Result<(), String> {
  STORE.with(|store| {
      let mut store = store.borrow_mut();
      if let Some(mut charge) = store.get_charge(id).cloned() {
          charge.status = status;
          store.update_charge(id.to_string(), charge);
          Ok(())
      } else {
          Err("Charge not found".to_string())
      }
  })
}
