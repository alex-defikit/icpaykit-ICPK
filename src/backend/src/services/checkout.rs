use ic_cdk::api::time;
use ic_cdk::export::Principal;

use crate::models::checkout::{Checkout, CheckoutCreate, CheckoutStatus};
use crate::state::store::STORE;

pub fn create_checkout(merchant_id: Principal, params: CheckoutCreate) -> Checkout {
    let canister_id = ic_cdk::api::id();
    let id = generate_id();

    let checkout = Checkout {
        id: id.clone(),
        payment_link: format!("https://{}.icp0.io/{}", canister_id.to_text(), id), // Root path
        merchant_id,
        name: params.name,
        description: params.description,
        pricing_type: params.pricing_type,
        local_price: params.local_price,
        requested_info: params.requested_info,
        created_at: time(),
        updated_at: time(),
        status: CheckoutStatus::Pending,
    };

    STORE.with(|store| {
        store.borrow_mut().create_checkout(checkout.clone());
    });

    checkout
}

// Helper functions
fn generate_id() -> String {
    use sha2::{Sha256, Digest};
    let timestamp = time().to_string();
    let mut hasher = Sha256::new();
    hasher.update(timestamp.as_bytes());
    hex::encode(&hasher.finalize()[..16])
}
