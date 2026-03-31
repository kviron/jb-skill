use tauri::State;

use crate::core::{
    app_state::AppState,
    contracts::{err, ok, ApiResponse, ConflictRecord, InstallResult, ModRecord, OperationResult, SwitchProfileResult},
    errors::ApiError,
    events,
    plugins,
    use_cases,
};

#[tauri::command]
pub fn core_list_mods(state: State<'_, AppState>, game_id: String) -> ApiResponse<Vec<ModRecord>> {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    match use_cases::list_mods(&db.conn, &game_id) {
        Ok(data) => ok(data),
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            err(&api_err.code, &api_err.message, api_err.recoverable)
        }
    }
}

#[tauri::command]
pub fn core_get_conflicts(
    state: State<'_, AppState>,
    profile_id: String,
) -> ApiResponse<Vec<ConflictRecord>> {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    match use_cases::get_conflicts(&db.conn, &profile_id) {
        Ok(data) => ok(data),
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            err(&api_err.code, &api_err.message, api_err.recoverable)
        }
    }
}

#[tauri::command]
pub fn core_install_mod_from_archive(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    game_id: String,
    profile_id: String,
    archive_path: String,
) -> ApiResponse<InstallResult> {
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_install_will_start(&app, "pending", &profile_id);
    events::emit_deploy_will_start(&app, "pending", &profile_id);
    let result = match use_cases::install_mod_from_archive(&mut db.conn, &game_id, &profile_id, &archive_path) {
        Ok(value) => value,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            events::emit_operation_failed(
                &app,
                "pending",
                "install",
                &api_err.code,
                &api_err.message,
                api_err.recoverable,
            );
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    let count = match use_cases::recalc_conflicts(&db.conn, &profile_id) {
        Ok(count) => count,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    events::emit_conflicts_recalculated(&app, &profile_id, count);
    events::emit_deploy_did_finish(&app, &result.operation_id, &profile_id);
    events::emit_install_did_finish(&app, &result.operation_id, &profile_id, &result.mod_id);
    ok(result)
}

#[tauri::command]
pub fn core_set_mod_enabled(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    profile_id: String,
    mod_id: String,
    enabled: bool,
) -> ApiResponse<OperationResult> {
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_deploy_will_start(&app, "pending", &profile_id);
    let result = match use_cases::set_mod_enabled(&mut db.conn, &profile_id, &mod_id, enabled) {
        Ok(value) => value,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            events::emit_operation_failed(
                &app,
                "pending",
                "deploy",
                &api_err.code,
                &api_err.message,
                api_err.recoverable,
            );
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    let count = match use_cases::recalc_conflicts(&db.conn, &profile_id) {
        Ok(count) => count,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    events::emit_conflicts_recalculated(&app, &profile_id, count);
    events::emit_deploy_did_finish(&app, &result.operation_id, &profile_id);
    ok(result)
}

#[tauri::command]
pub fn core_remove_mod(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    profile_id: String,
    mod_id: String,
) -> ApiResponse<OperationResult> {
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_deploy_will_start(&app, "pending", &profile_id);
    let result = match use_cases::remove_mod(&mut db.conn, &profile_id, &mod_id) {
        Ok(value) => value,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            events::emit_operation_failed(
                &app,
                "pending",
                "deploy",
                &api_err.code,
                &api_err.message,
                api_err.recoverable,
            );
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    let count = match use_cases::recalc_conflicts(&db.conn, &profile_id) {
        Ok(count) => count,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    events::emit_conflicts_recalculated(&app, &profile_id, count);
    events::emit_deploy_did_finish(&app, &result.operation_id, &profile_id);
    ok(result)
}

#[tauri::command]
pub fn core_switch_profile(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    game_id: String,
    target_profile_id: String,
) -> ApiResponse<SwitchProfileResult> {
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_profile_will_change(&app, "pending", &target_profile_id);
    events::emit_deploy_will_start(&app, "pending", &target_profile_id);
    let result = match use_cases::switch_profile(&mut db.conn, &game_id, &target_profile_id) {
        Ok(value) => value,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            events::emit_operation_failed(
                &app,
                "pending",
                "profile-switch",
                &api_err.code,
                &api_err.message,
                api_err.recoverable,
            );
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    let count = match use_cases::recalc_conflicts(&db.conn, &target_profile_id) {
        Ok(count) => count,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    events::emit_conflicts_recalculated(&app, &target_profile_id, count);
    events::emit_deploy_did_finish(&app, &result.operation_id, &target_profile_id);
    events::emit_profile_did_change(&app, &result.operation_id, &target_profile_id);
    ok(result)
}

#[tauri::command]
pub fn core_plugin_smoke_test() -> ApiResponse<plugins::SmokeResult> {
    let manifest_path = plugins::plugin_root().join("game-pilot").join("manifest.json");
    let manifest = match plugins::load_manifest(&manifest_path) {
        Ok(manifest) => manifest,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    match plugins::run_smoke_test(&manifest, "sample.zip") {
        Ok(smoke) => ok(smoke),
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            err(&api_err.code, &api_err.message, api_err.recoverable)
        }
    }
}
