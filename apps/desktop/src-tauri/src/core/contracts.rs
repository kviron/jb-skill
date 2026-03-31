use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub data: T,
}

pub fn ok<T>(data: T) -> ApiResponse<T> {
    ApiResponse { ok: true, data }
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
