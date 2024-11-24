use std::cell::RefCell;
use std::collections::HashMap;
use candid::Principal;
use ic_ledger_types::AccountIdentifier;

use crate::models::checkout::Checkout;
use crate::models::charge::Charge;
use crate::models::webhook::WebhookConfig;
use crate::models::merchant::*;
use crate::models::payment::Payment;

thread_local! {
    pub static STORE: RefCell<Store> = RefCell::new(Store::init());
}

#[derive(Clone)]
pub struct MerchantAddresses {
    pub btc_address: String,
}

#[derive(Default)]
pub struct Store {
    merchants: HashMap<Principal, bool>,
    merchant_addresses: HashMap<Principal, MerchantAddresses>,
    transaction_subaccounts: HashMap<String, [u8; 32]>,
    checkouts: HashMap<String, Checkout>,
    charges: HashMap<String, Charge>,
    webhook_configs: HashMap<Principal, WebhookConfig>,
    payments: HashMap<String, Payment>,
}

impl Store {
    pub fn init() -> Self {
        Self {
            merchants: HashMap::new(),
            merchant_addresses: HashMap::new(),
            transaction_subaccounts: HashMap::new(),
            checkouts: HashMap::new(),
            charges: HashMap::new(),
            webhook_configs: HashMap::new(),
            payments: HashMap::new(),
        }
    }

    // Thêm phương thức để đăng ký subaccount cho giao dịch
    pub fn register_transaction_subaccount(&mut self, charge_id: String, subaccount: [u8; 32]) {
        self.transaction_subaccounts.insert(charge_id, subaccount);
    }

    // Thêm phương thức để lấy subaccount của giao dịch
    pub fn get_transaction_subaccount(&self, charge_id: &String) -> Option<[u8; 32]> {
        self.transaction_subaccounts.get(charge_id).cloned()
    }

    pub fn register_merchant(&mut self, principal: Principal) {
        self.merchants.insert(principal, true);
        // self.merchant_accounts.insert(principal, account)
    }

    pub fn is_merchant(&self, principal: &Principal) -> bool {
        self.merchants.get(principal).copied().unwrap_or(false)
    }

    // pub fn get_merchant_account(&self, merchant_id: &Principal) {
    //     self.merchant_accounts.get(merchant_id)
    // }
    pub fn create_checkout(&mut self, checkout: Checkout) {
        self.checkouts.insert(checkout.id.clone(), checkout);
    }

    pub fn get_checkout(&self, id: &str) -> Option<&Checkout> {
        self.checkouts.get(id)
    }

    pub fn update_checkout(&mut self, id: &str, checkout: Checkout) -> Option<Checkout> {
        self.checkouts.insert(id.to_string(), checkout)
    }

    pub fn delete_checkout(&mut self, id: &str) -> Option<Checkout> {
        self.checkouts.remove(id)
    }

    pub fn create_charge(&mut self, charge: Charge) {
        self.charges.insert(charge.id.clone(), charge);
    }

    pub fn get_charge(&self, id: &str) -> Option<&Charge> {
        self.charges.get(id)
    }

    pub fn list_charges(&self) -> Vec<&Charge> {
        self.charges.values().collect()
    }

    pub fn update_charge(&mut self, id: String, charge: Charge) {
        self.charges.insert(id, charge);
    }

    pub fn unregister_merchant(&mut self, merchant_id: Principal) {
        self.merchants.remove(&merchant_id);
    }

    pub fn register_webhook(&mut self, merchant_id: Principal, config: WebhookConfig) {
        self.webhook_configs.insert(merchant_id, config);
    }

    pub fn get_webhook_config(&self, merchant_id: &Principal) -> Option<&WebhookConfig> {
        self.webhook_configs.get(merchant_id)
    }

    pub fn remove_webhook_config(&mut self, merchant_id: &Principal) {
        self.webhook_configs.remove(merchant_id);
    }

    pub fn register_merchant_addresses(
        &mut self, 
        merchant_id: Principal,
        btc_address: String
    ) {
        // Ensure merchant is registered first
        self.merchants.insert(merchant_id, true);
        
        // Store merchant addresses
        self.merchant_addresses.insert(merchant_id, MerchantAddresses {
            btc_address,
        });
    }

    pub fn get_merchant_addresses(&self, merchant_id: &Principal) -> Option<&MerchantAddresses> {
        self.merchant_addresses.get(merchant_id)
    }

    pub fn create_payment(&mut self, payment: Payment) {
        self.payments.insert(payment.id.clone(), payment);
    }

    pub fn get_payment(&self, payment_id: &str) -> Option<&Payment> {
        self.payments.get(payment_id)
    }

    pub fn update_payment(&mut self, payment_id: &str, payment: Payment) {
        self.payments.insert(payment_id.to_string(), payment);
    }
}
