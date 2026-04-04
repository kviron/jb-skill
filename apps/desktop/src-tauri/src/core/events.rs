use tauri::Emitter;

use chrono::Utc;
use serde_json::json;

use crate::core::contracts::DomainEvent;

fn emit_domain_event(
    app: &tauri::AppHandle,
    name: &str,
    operation_id: &str,
    profile_id: Option<&str>,
    payload: serde_json::Value,
) {
    let _ = app.emit(
        name,
        DomainEvent {
            name: name.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            operation_id: operation_id.to_string(),
            profile_id: profile_id.map(ToString::to_string),
            payload,
        },
    );
}

pub fn emit_install_will_start(app: &tauri::AppHandle, operation_id: &str, profile_id: &str) {
    emit_domain_event(app, "install:will-start", operation_id, Some(profile_id), json!({}));
}

pub fn emit_install_did_finish(app: &tauri::AppHandle, operation_id: &str, profile_id: &str, mod_id: &str) {
    emit_domain_event(
        app,
        "install:did-finish",
        operation_id,
        Some(profile_id),
        json!({ "modId": mod_id }),
    );
}

pub fn emit_deploy_will_start(app: &tauri::AppHandle, operation_id: &str, profile_id: &str) {
    emit_domain_event(app, "deploy:will-start", operation_id, Some(profile_id), json!({}));
}

pub fn emit_deploy_did_finish(app: &tauri::AppHandle, operation_id: &str, profile_id: &str) {
    emit_domain_event(app, "deploy:did-finish", operation_id, Some(profile_id), json!({}));
}

pub fn emit_profile_will_change(app: &tauri::AppHandle, operation_id: &str, profile_id: &str) {
    emit_domain_event(app, "profile:will-change", operation_id, Some(profile_id), json!({}));
}

pub fn emit_profile_did_change(app: &tauri::AppHandle, operation_id: &str, profile_id: &str) {
    emit_domain_event(app, "profile:did-change", operation_id, Some(profile_id), json!({}));
}

pub fn emit_conflicts_recalculated(app: &tauri::AppHandle, profile_id: &str, count: usize) {
    emit_domain_event(
        app,
        "conflicts:recalculated",
        "system",
        Some(profile_id),
        json!({ "count": count }),
    );
}

pub fn emit_operation_failed(
    app: &tauri::AppHandle,
    operation_id: &str,
    stage: &str,
    error_code: &str,
    message: &str,
    recoverable: bool,
) {
    emit_domain_event(
        app,
        "operation:failed",
        operation_id,
        None,
        json!({
            "stage": stage,
            "errorCode": error_code,
            "message": message,
            "recoverable": recoverable
        }),
    );
}
