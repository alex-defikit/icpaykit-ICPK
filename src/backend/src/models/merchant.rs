use candid::{CandidType, Deserialize, Principal};
use ic_ledger_types::AccountIdentifier;

#[derive(CandidType, Deserialize, Clone)]
pub struct Merchant {
    pub principal: Principal,
    pub icp_account: AccountIdentifier,
    pub btc_addresses: BtcAddresses,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct BtcAddresses {
    pub p2pkh: Option<String>,
    pub p2tr_raw_key_spend: Option<String>,
    pub p2tr_script_spend: Option<String>,
}