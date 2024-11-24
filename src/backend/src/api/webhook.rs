use crate::models::webhook::{WebhookConfig, WebhookEvent};
use crate::services::webhook as webhook_service;
use crate::auth::merchant::verify_merchant;


pub async fn register_webhook(config: WebhookConfig) -> Result<(), String> {
    let merchant_id = ic_cdk::caller();
    
    if !verify_merchant(&merchant_id) {
        return Err("Unauthorized: Not a registered merchant".to_string());
    }

    webhook_service::register_webhook(merchant_id, config);
    Ok(())
}

pub async fn verify_webhook(payload: String, signature: String) -> Result<WebhookEvent, String> {
    let merchant_id = ic_cdk::caller();
    
    let config = webhook_service::get_webhook_config(&merchant_id)
        .ok_or("Webhook config not found")?;

    webhook_service::verify_event_body(&payload, &signature, &config.secret)
}

pub fn get_webhook_config() -> Result<WebhookConfig, String> {
    let merchant_id = ic_cdk::caller();
    
    webhook_service::get_webhook_config(&merchant_id)
        .ok_or("Webhook config not found".to_string())
}
