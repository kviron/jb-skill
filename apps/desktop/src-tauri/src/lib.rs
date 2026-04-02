mod core;

use crate::core::{app_state::AppState, db::CoreDb, tx};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use tauri::Emitter;
use tauri::Manager;

fn setup_err(context: &str, source: impl std::fmt::Display) -> io::Error {
    let msg = format!("{context}: {source}");
    eprintln!("[pantheon] {msg}");
    io::Error::new(io::ErrorKind::Other, msg)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data = app
                .path()
                .app_data_dir()
                .map_err(|e| setup_err("app_data_dir", e))?
                .join("pantheon");
            let staging_root = app_data.join("staging");
            std::fs::create_dir_all(&staging_root).map_err(|e| setup_err("create staging_dir", e))?;

            let db_path = app_data.join("core.db");
            let db = CoreDb::new(db_path).map_err(|e| setup_err("CoreDb::new", e))?;
            if let Err(e) = tx::recover_started_transactions(&db.conn) {
                eprintln!("[pantheon] recover_started_transactions: {e}");
            }
            let plugin_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("..")
                .join("plugins")
                .join("game-pilot")
                .join("manifest.json");
            let manifest = core::plugins::load_manifest(&plugin_manifest)
                .map_err(|e| setup_err("load_plugin_manifest", e))?;
            let _ = core::plugins::run_smoke_test(&manifest, "bootstrap.zip")
                .map_err(|e| setup_err("plugin_smoke_test", e))?;
            app.manage(AppState {
                db: std::sync::Mutex::new(db),
                staging_root,
                install_sessions: std::sync::Mutex::new(HashMap::new()),
            });
            let _ = app.emit("app-ready", serde_json::Value::Null);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            core::commands::core_prepare_mod_install,
            core::commands::core_finalize_mod_install,
            core::commands::core_cancel_mod_install_session,
            core::commands::core_install_mod_from_archive,
            core::commands::core_set_mod_enabled,
            core::commands::core_remove_mod,
            core::commands::core_switch_profile,
            core::commands::core_list_mods,
            core::commands::core_get_conflicts,
            core::commands::core_reorder_mod_priority,
            core::commands::core_plugin_smoke_test,
            core::commands::close_splash,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
