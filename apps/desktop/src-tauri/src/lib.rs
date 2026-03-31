mod core;

use crate::core::{app_state::AppState, db::CoreDb, tx};
use std::path::PathBuf;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app
                .path()
                .app_data_dir()
                .map_err(|_| tauri::Error::AssetNotFound("app_data_dir".into()))?
                .join("jb-skill")
                .join("core.db");
            let db = CoreDb::new(path).map_err(|_| tauri::Error::AssetNotFound("db_init".into()))?;
            let _ = tx::recover_started_transactions(&db.conn);
            let plugin_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("..")
                .join("plugins")
                .join("game-pilot")
                .join("manifest.json");
            let manifest = core::plugins::load_manifest(&plugin_manifest)
                .map_err(|_| tauri::Error::AssetNotFound("plugin_manifest".into()))?;
            let _ = core::plugins::run_smoke_test(&manifest, "bootstrap.zip")
                .map_err(|_| tauri::Error::AssetNotFound("plugin_smoke".into()))?;
            app.manage(AppState {
                db: std::sync::Mutex::new(db),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            core::commands::core_install_mod_from_archive,
            core::commands::core_set_mod_enabled,
            core::commands::core_remove_mod,
            core::commands::core_switch_profile,
            core::commands::core_list_mods,
            core::commands::core_get_conflicts,
            core::commands::core_plugin_smoke_test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
