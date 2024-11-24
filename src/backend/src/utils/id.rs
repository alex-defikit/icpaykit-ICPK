use ic_cdk::api::time;

pub fn generate_id() -> String {
    let timestamp = time();
    let random = ic_cdk::api::id();
    format!("{}_{}", timestamp, random.to_string())
} 