use tauri::Manager;
use tauri::State;

use crate::core::{
    app_state::AppState,
    contracts::{
        err, ok, ApiResponse, ConflictRecord, InstallResult, ModRecord, OperationResult, PrepareModInstallResult,
        SwitchProfileResult,
    },
    errors::ApiError,
    events,
    fomod::FomodSelections,
    plugins,
    use_cases,
};

fn conflict_count(conn: &rusqlite::Connection, profile_id: &str) -> Result<usize, crate::core::errors::CoreError> {
    let n: i64 = conn.query_row(
        "SELECT COUNT(*) FROM conflicts WHERE profile_id = ?1",
        [profile_id],
        |row| row.get(0),
    )?;
    Ok(n as usize)
}

#[tauri::command]
pub fn core_list_mods(
    state: State<'_, AppState>,
    game_id: String,
    profile_id: String,
) -> ApiResponse<Vec<ModRecord>> {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    match use_cases::list_mods(&db.conn, &game_id, &profile_id) {
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
pub fn core_prepare_mod_install(
    state: State<'_, AppState>,
    archive_path: String,
) -> ApiResponse<PrepareModInstallResult> {
    let staging = state.staging_root.clone();
    match use_cases::prepare_mod_install(staging.as_path(), &archive_path, &state.install_sessions) {
        Ok(data) => ok(data),
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            err(&api_err.code, &api_err.message, api_err.recoverable)
        }
    }
}

#[tauri::command]
pub fn core_finalize_mod_install(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    game_id: String,
    profile_id: String,
    session_id: String,
    fomod_selections: Option<serde_json::Value>,
) -> ApiResponse<InstallResult> {
    let fomod_sel: Option<FomodSelections> = match fomod_selections {
        None => None,
        Some(v) => match serde_json::from_value(v) {
            Ok(s) => Some(s),
            Err(_) => return err("INVALID_INPUT", "Invalid FOMOD selections", true),
        },
    };
    let staging = state.staging_root.clone();
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_install_will_start(&app, "pending", &profile_id);
    events::emit_deploy_will_start(&app, "pending", &profile_id);
    let result = match use_cases::finalize_mod_install(
        &mut db.conn,
        &game_id,
        &profile_id,
        &session_id,
        fomod_sel,
        staging.as_path(),
        &state.install_sessions,
    ) {
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
    let count = match conflict_count(&db.conn, &profile_id) {
        Ok(c) => c,
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
pub fn core_cancel_mod_install_session(
    state: State<'_, AppState>,
    session_id: String,
) -> ApiResponse<()> {
    let staging = state.staging_root.clone();
    match use_cases::cancel_mod_install_session(staging.as_path(), &session_id, &state.install_sessions) {
        Ok(()) => ok(()),
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
    let staging = state.staging_root.clone();
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_install_will_start(&app, "pending", &profile_id);
    events::emit_deploy_will_start(&app, "pending", &profile_id);
    let result = match use_cases::install_mod_from_archive(
        &mut db.conn,
        &game_id,
        &profile_id,
        &archive_path,
        staging.as_path(),
    ) {
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
    let count = match conflict_count(&db.conn, &profile_id) {
        Ok(c) => c,
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
    let staging = state.staging_root.clone();
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_deploy_will_start(&app, "pending", &profile_id);
    let result = match use_cases::set_mod_enabled(&mut db.conn, &profile_id, &mod_id, enabled, staging.as_path()) {
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
    let count = match conflict_count(&db.conn, &profile_id) {
        Ok(c) => c,
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
    let staging = state.staging_root.clone();
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_deploy_will_start(&app, "pending", &profile_id);
    let result = match use_cases::remove_mod(&mut db.conn, &profile_id, &mod_id, staging.as_path()) {
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
    let count = match conflict_count(&db.conn, &profile_id) {
        Ok(c) => c,
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
    let staging = state.staging_root.clone();
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_profile_will_change(&app, "pending", &target_profile_id);
    events::emit_deploy_will_start(&app, "pending", &target_profile_id);
    let result = match use_cases::switch_profile(&mut db.conn, &game_id, &target_profile_id, staging.as_path()) {
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
    let count = match conflict_count(&db.conn, &target_profile_id) {
        Ok(c) => c,
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
pub fn core_reorder_mod_priority(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    profile_id: String,
    mod_id: String,
    move_up: bool,
) -> ApiResponse<OperationResult> {
    let staging = state.staging_root.clone();
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return err("INVALID_INPUT", "Invalid input", true),
    };
    events::emit_deploy_will_start(&app, "pending", &profile_id);
    let result = match use_cases::reorder_mod_priority(&mut db.conn, &profile_id, &mod_id, move_up, staging.as_path()) {
        Ok(value) => value,
        Err(core_err) => {
            let api_err = ApiError::from_core(core_err);
            events::emit_operation_failed(
                &app,
                "pending",
                "reorder",
                &api_err.code,
                &api_err.message,
                api_err.recoverable,
            );
            return err(&api_err.code, &api_err.message, api_err.recoverable);
        }
    };
    let count = match conflict_count(&db.conn, &profile_id) {
        Ok(c) => c,
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
pub fn close_splash(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(splash) = app.get_webview_window("splash") {
        let _ = splash.close();
    }
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.show();
        let _ = main.set_focus();
    }
    Ok(())
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
