use std::sync::Mutex;

use crate::core::db::CoreDb;

pub struct AppState {
    pub db: Mutex<CoreDb>,
}
