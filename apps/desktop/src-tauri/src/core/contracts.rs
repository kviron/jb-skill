use serde::Serialize;
use serde_json::Value;

use super::fomod::FomodWizardPayload;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub data: Option<T>,
    pub error: Option<ApiErrorPayload>,
}

pub fn ok<T>(data: T) -> ApiResponse<T> {
    ApiResponse {
        ok: true,
        data: Some(data),
        error: None,
    }
}

pub fn err<T>(code: &str, message: &str, recoverable: bool) -> ApiResponse<T> {
    ApiResponse {
        ok: false,
        data: None,
        error: Some(ApiErrorPayload {
            code: code.to_string(),
            message: message.to_string(),
            recoverable,
        }),
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorPayload {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationResult {
    pub operation_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallResult {
    pub operation_id: String,
    pub mod_id: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareModInstallResult {
    pub session_id: String,
    /// `"plain"` | `"fomod"`
    pub kind: String,
    pub wizard: Option<FomodWizardPayload>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchProfileResult {
    pub operation_id: String,
    pub active_profile_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModRecord {
    pub id: String,
    pub game_id: String,
    pub name: String,
    pub version: Option<String>,
    pub archive_path: String,
    pub enabled: bool,
    pub priority: i64,
    pub installed_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictRecord {
    pub id: String,
    pub profile_id: String,
    pub target_path: String,
    pub winner_mod_id: String,
    pub loser_mod_ids_json: String,
    pub resolved_by: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationEvent {
    pub operation_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictRecalculatedEvent {
    pub profile_id: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationFailedEvent {
    pub operation_id: String,
    pub stage: String,
    pub error_code: String,
    pub message: String,
    pub recoverable: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainEvent {
    pub name: String,
    pub timestamp: String,
    pub operation_id: String,
    pub profile_id: Option<String>,
    pub payload: Value,
}
