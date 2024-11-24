use sha2::{Sha256, Digest};
use hex;
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{
    HttpResponse, CanisterHttpRequestArgument, HttpMethod, HttpHeader
};

use crate::models::webhook::{WebhookEvent, WebhookConfig, EventType, EventData};
use crate::state::store::STORE;
use crate::utils::generate_id;

pub fn verify_signature(payload: &[u8], signature: &str, secret: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    hasher.update(secret.as_bytes());
    let computed_signature = hex::encode(hasher.finalize());
    
    computed_signature == signature
}

pub fn verify_event_body(payload: &str, signature: &str, secret: &str) -> Result<WebhookEvent, String> {
    let data: serde_json::Value = serde_json::from_str(payload)
        .map_err(|_| "Invalid payload provided. No JSON object could be decoded".to_string())?;
    
    if !data.get("event").is_some() {
        return Err("Invalid payload provided. No event object found".to_string());
    }

    if !verify_signature(payload.as_bytes(), signature, secret) {
        return Err("Invalid signature".to_string());
    }

    let event: WebhookEvent = serde_json::from_value(data["event"].clone())
        .map_err(|_| "Failed to parse event data".to_string())?;

    Ok(event)
}

pub fn register_webhook(merchant_id: Principal, config: WebhookConfig) {
    STORE.with(|store| {
        store.borrow_mut().register_webhook(merchant_id, config);
    });
}

pub fn get_webhook_config(merchant_id: &Principal) -> Option<WebhookConfig> {
    STORE.with(|store| {
        store.borrow().get_webhook_config(merchant_id).cloned()
    })
}

fn sign_payload(event: &WebhookEvent, secret: &str) -> Result<String, String> {
    let payload = serde_json::to_string(event)
        .map_err(|_| "Failed to serialize event".to_string())?;
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    hasher.update(secret.as_bytes());
    Ok(hex::encode(hasher.finalize()))
}

pub async fn send_webhook_notification(
    merchant_id: &Principal,
    event_type: EventType,
    data: EventData
) -> Result<(), String> {
    let config = get_webhook_config(merchant_id)
        .ok_or("Webhook config not found")?;

    if !config.enabled {
        return Ok(());
    }

    let event = WebhookEvent {
        id: generate_id(),
        api_version: "2024-03".to_string(),
        event_type: event_type,
        created_at: ic_cdk::api::time().to_string(),
        data: data,
        resource: "charge".to_string(),
    };

    let body = serde_json::to_string(&event)
        .map_err(|e| format!("Failed to serialize event: {}", e))?;
    
    let signature = sign_payload(&event, &config.secret)?;

    let request = CanisterHttpRequestArgument {
        url: config.url,
        method: HttpMethod::POST,
        body: Some(body.into_bytes()),
        max_response_bytes: Some(2048), // Adjust as needed
        transform: None,
        headers: vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "X-Webhook-Signature".to_string(),
                value: signature,
            },
        ],
    };

    // Send the HTTP request
    let response: HttpResponse = match ic_cdk::api::call::call_with_payment(
        ic_cdk::export::Principal::management_canister(),
        "http_request",
        (request,),
        0,
    ).await {
        Ok((response,)) => response,
        Err((code, msg)) => {
            return Err(format!("HTTP request failed with code {:?}: {}", code, msg));
        }
    };

    // Check response status
    if response.status != 200 {
        return Err(format!(
            "Webhook notification failed with status code: {}",
            response.status
        ));
    }

    Ok(())
}