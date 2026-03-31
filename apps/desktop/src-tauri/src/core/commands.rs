use tauri::State;

use crate::core::{
    app_state::AppState,
    contracts::{ok, ApiResponse, ConflictRecord, InstallResult, ModRecord, OperationResult, SwitchProfileResult},
    errors::{ApiError, CoreError},
    events,
    plugins,
    use_cases,
};

#[tauri::command]
pub fn core_list_mods(state: State<'_, AppState>, game_id: String) -> Result<ApiResponse<Vec<ModRecord>>, ApiError> {
    let db = state.db.lock().map_err(|_| ApiError::from(CoreError::InvalidInput))?;
    let data = use_cases::list_mods(&db.conn, &game_id).map_err(ApiError::from)?;
    Ok(ok(data))
}

#[tauri::command]
pub fn core_get_conflicts(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<ApiResponse<Vec<ConflictRecord>>, ApiError> {
    let db = state.db.lock().map_err(|_| ApiError::from(CoreError::InvalidInput))?;
    let data = use_cases::get_conflicts(&db.conn, &profile_id).map_err(ApiError::from)?;
    Ok(ok(data))
}

#[tauri::command]
pub fn core_install_mod_from_archive(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    game_id: String,
    profile_id: String,
    archive_path: String,
) -> Result<ApiResponse<InstallResult>, ApiError> {
    let mut db = state.db.lock().map_err(|_| ApiError::from(CoreError::InvalidInput))?;
    events::emit_install_will_start(&app, "pending");
    events::emit_deploy_will_start(&app, "pending");
    let result = match use_cases::install_mod_from_archive(&mut db.conn, &game_id, &profile_id, &archive_path) {
        Ok(value) => value,
        Err(err) => {
            let api_err = ApiError::from(err);
            events::emit_operation_failed(
                &app,
                "pending",
                "install",
                &api_err.code,
                &api_err.message,
                api_err.recoverable,
            );
            return Err(api_err);
        }
    };
    let count = use_cases::recalc_conflicts(&db.conn, &profile_id).map_err(ApiError::from)?;
    events::emit_conflicts_recalculated(&app, &profile_id, count);
    events::emit_deploy_did_finish(&app, &result.operation_id);
    events::emit_install_did_finish(&app, &result.operation_id);
    Ok(ok(result))
}

#[tauri::command]
pub fn core_set_mod_enabled(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    profile_id: String,
    mod_id: String,
    enabled: bool,
) -> Result<ApiResponse<OperationResult>, ApiError> {
    let mut db = state.db.lock().map_err(|_| ApiError::from(CoreError::InvalidInput))?;
    events::emit_deploy_will_start(&app, "pending");
    let result = match use_cases::set_mod_enabled(&mut db.conn, &profile_id, &mod_id, enabled) {
        Ok(value) => value,
        Err(err) => {
            let api_err = ApiError::from(err);
            events::emit_operation_failed(
                &app,
                "pending",
                "deploy",
                &api_err.code,
                &api_err.message,
                api_err.recoverable,
            );
            return Err(api_err);
        }
    };
    let count = use_cases::recalc_conflicts(&db.conn, &profile_id).map_err(ApiError::from)?;
    events::emit_conflicts_recalculated(&app, &profile_id, count);
    events::emit_deploy_did_finish(&app, &result.operation_id);
    Ok(ok(result))
}

#[tauri::command]
pub fn core_remove_mod(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    profile_id: String,
    mod_id: String,
) -> Result<ApiResponse<OperationResult>, ApiError> {
    let mut db = state.db.lock().map_err(|_| ApiError::from(CoreError::InvalidInput))?;
    events::emit_deploy_will_start(&app, "pending");
    let result = match use_cases::remove_mod(&mut db.conn, &profile_id, &mod_id) {
        Ok(value) => value,
        Err(err) => {
            let api_err = ApiError::from(err);
            events::emit_operation_failed(
                &app,
                "pending",
                "deploy",
                &api_err.code,
                &api_err.message,
                api_err.recoverable,
            );
            return Err(api_err);
        }
    };
    let count = use_cases::recalc_conflicts(&db.conn, &profile_id).map_err(ApiError::from)?;
    events::emit_conflicts_recalculated(&app, &profile_id, count);
    events::emit_deploy_did_finish(&app, &result.operation_id);
    Ok(ok(result))
}

#[tauri::command]
pub fn core_switch_profile(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    game_id: String,
    target_profile_id: String,
) -> Result<ApiResponse<SwitchProfileResult>, ApiError> {
    let mut db = state.db.lock().map_err(|_| ApiError::from(CoreError::InvalidInput))?;
    events::emit_profile_will_change(&app, "pending");
    events::emit_deploy_will_start(&app, "pending");
    let result = match use_cases::switch_profile(&mut db.conn, &game_id, &target_profile_id) {
        Ok(value) => value,
        Err(err) => {
            let api_err = ApiError::from(err);
            events::emit_operation_failed(
                &app,
                "pending",
                "profile-switch",
                &api_err.code,
                &api_err.message,
                api_err.recoverable,
            );
            return Err(api_err);
        }
    };
    let count = use_cases::recalc_conflicts(&db.conn, &target_profile_id).map_err(ApiError::from)?;
    events::emit_conflicts_recalculated(&app, &target_profile_id, count);
    events::emit_deploy_did_finish(&app, &result.operation_id);
    events::emit_profile_did_change(&app, &result.operation_id);
    Ok(ok(result))
}

#[tauri::command]
pub fn core_plugin_smoke_test() -> Result<ApiResponse<plugins::SmokeResult>, ApiError> {
    let manifest_path = plugins::plugin_root().join("game-pilot").join("manifest.json");
    let manifest = plugins::load_manifest(&manifest_path).map_err(ApiError::from)?;
    let smoke = plugins::run_smoke_test(&manifest, "sample.zip").map_err(ApiError::from)?;
    Ok(ok(smoke))
}
