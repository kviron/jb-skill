use std::path::{Path, PathBuf};

use crate::core::errors::CoreError;

/// Relative posix path to `ModuleConfig.xml` inside staging, if present.
pub fn find_module_config_relative(staging_root: &Path) -> Result<Option<String>, CoreError> {
    let a = PathBuf::from("fomod/ModuleConfig.xml");
    let b = PathBuf::from("ModuleConfig.xml");
    if staging_root.join(&a).is_file() {
        return Ok(Some(a.to_string_lossy().replace('\\', "/")));
    }
    if staging_root.join(&b).is_file() {
        return Ok(Some(b.to_string_lossy().replace('\\', "/")));
    }
    Ok(None)
}

pub fn module_config_abs_path(staging_root: &Path, rel: &str) -> PathBuf {
    staging_root.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR))
}
