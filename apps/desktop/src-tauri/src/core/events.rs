use tauri::Emitter;

use crate::core::contracts::{ConflictRecalculatedEvent, OperationEvent, OperationFailedEvent};

pub fn emit_install_will_start(app: &tauri::AppHandle, operation_id: &str) {
    let _ = app.emit(
        "install.will-start",
        OperationEvent {
            operation_id: operation_id.to_string(),
            message: "Install started".to_string(),
        },
    );
}

pub fn emit_install_did_finish(app: &tauri::AppHandle, operation_id: &str) {
    let _ = app.emit(
        "install.did-finish",
        OperationEvent {
            operation_id: operation_id.to_string(),
            message: "Install finished".to_string(),
        },
    );
}

pub fn emit_deploy_will_start(app: &tauri::AppHandle, operation_id: &str) {
    let _ = app.emit(
        "deploy.will-start",
        OperationEvent {
            operation_id: operation_id.to_string(),
            message: "Deploy started".to_string(),
        },
    );
}

pub fn emit_deploy_did_finish(app: &tauri::AppHandle, operation_id: &str) {
    let _ = app.emit(
        "deploy.did-finish",
        OperationEvent {
            operation_id: operation_id.to_string(),
            message: "Deploy finished".to_string(),
        },
    );
}

pub fn emit_profile_will_change(app: &tauri::AppHandle, operation_id: &str) {
    let _ = app.emit(
        "profile.will-change",
        OperationEvent {
            operation_id: operation_id.to_string(),
            message: "Switching profile".to_string(),
        },
    );
}

pub fn emit_profile_did_change(app: &tauri::AppHandle, operation_id: &str) {
    let _ = app.emit(
        "profile.did-change",
        OperationEvent {
            operation_id: operation_id.to_string(),
            message: "Profile switched".to_string(),
        },
    );
}

pub fn emit_conflicts_recalculated(app: &tauri::AppHandle, profile_id: &str, count: usize) {
    let _ = app.emit(
        "conflicts.recalculated",
        ConflictRecalculatedEvent {
            profile_id: profile_id.to_string(),
            count,
        },
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
    let _ = app.emit(
        "operation.failed",
        OperationFailedEvent {
            operation_id: operation_id.to_string(),
            stage: stage.to_string(),
            error_code: error_code.to_string(),
            message: message.to_string(),
            recoverable,
        },
    );
}
