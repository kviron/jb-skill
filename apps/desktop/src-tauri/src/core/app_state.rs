use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::core::db::CoreDb;
use crate::core::install_session::InstallSession;

pub struct AppState {
    pub db: Mutex<CoreDb>,
    pub staging_root: PathBuf,
    pub install_sessions: Mutex<HashMap<String, InstallSession>>,
}
