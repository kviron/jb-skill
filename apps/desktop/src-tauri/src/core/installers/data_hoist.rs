use std::fs;
use std::path::Path;

use crate::core::errors::CoreError;
use crate::core::installers::ModInstaller;

/// If every file lives under a single top-level `Data/` folder, hoist contents to staging root
/// (common Gamebryo-style layout).
pub struct DataHoistInstaller;

fn first_component(path: &str) -> Option<&str> {
    let p = path.trim_start_matches('/');
    if p.is_empty() {
        return None;
    }
    p.split('/').next()
}

impl ModInstaller for DataHoistInstaller {
    fn id(&self) -> &'static str {
        "data-hoist"
    }

    fn priority(&self) -> i32 {
        100
    }

    fn test_supported(&self, relative_paths: &[String]) -> bool {
        let mut roots = std::collections::HashSet::new();
        for p in relative_paths {
            if p.ends_with('/') {
                if let Some(f) = first_component(p) {
                    roots.insert(f.to_ascii_lowercase());
                }
                continue;
            }
            let Some(f) = first_component(p) else {
                continue;
            };
            roots.insert(f.to_ascii_lowercase());
        }
        roots.len() == 1 && roots.contains("data")
    }

    fn install(&self, staging_mod_dir: &Path) -> Result<(), CoreError> {
        let data_dir = staging_mod_dir.join("Data");
        if !data_dir.is_dir() {
            let data_lower = staging_mod_dir.join("data");
            if data_lower.is_dir() {
                return hoist_from(&data_lower, staging_mod_dir);
            }
            return Err(CoreError::InvalidInput);
        }
        hoist_from(&data_dir, staging_mod_dir)
    }
}

fn hoist_from(data_dir: &Path, staging_mod_dir: &Path) -> Result<(), CoreError> {
    for entry in fs::read_dir(data_dir).map_err(|e| CoreError::Io(e.into()))? {
        let entry = entry.map_err(|e| CoreError::Io(e.into()))?;
        let dest = staging_mod_dir.join(entry.file_name());
        if dest.exists() {
            if dest.is_dir() {
                fs::remove_dir_all(&dest).map_err(|e| CoreError::Io(e.into()))?;
            } else {
                fs::remove_file(&dest).map_err(|e| CoreError::Io(e.into()))?;
            }
        }
        fs::rename(entry.path(), &dest).map_err(|e| CoreError::Io(e.into()))?;
    }
    fs::remove_dir(data_dir).map_err(|e| CoreError::Io(e.into()))?;
    Ok(())
}
