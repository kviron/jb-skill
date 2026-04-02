//! Vortex-style installer chain: ordered by priority, first `test_supported` wins, then mutates staging.
//!
//! Идентификаторы из манифеста плагина (`installers`) должны совпадать с [`INSTALLER_ID_*`].
use std::path::Path;

use crate::core::errors::CoreError;

mod basic;
mod data_hoist;

pub use basic::BasicInstaller;
pub use data_hoist::DataHoistInstaller;

pub const INSTALLER_ID_DATA_HOIST: &str = "data-hoist";
pub const INSTALLER_ID_BASIC: &str = "basic";

/// One mod installer (e.g. FOMOD-style in Vortex = separate module with priority).
pub trait ModInstaller {
    fn id(&self) -> &'static str;
    /// Higher runs first (like Vortex installer priority).
    fn priority(&self) -> i32;
    fn test_supported(&self, relative_paths: &[String]) -> bool;
    /// Transform extracted content under `staging_mod_dir` in place.
    fn install(&self, staging_mod_dir: &Path) -> Result<(), CoreError>;
}

/// Создать инсталлер по id из манифеста плагина.
pub fn boxed_installer(id: &str) -> Option<Box<dyn ModInstaller>> {
    match id.trim() {
        INSTALLER_ID_DATA_HOIST => Some(Box::new(DataHoistInstaller)),
        INSTALLER_ID_BASIC => Some(Box::new(BasicInstaller)),
        _ => None,
    }
}

pub fn installer_id_is_valid(id: &str) -> bool {
    boxed_installer(id).is_some()
}

/// Список id из манифеста (пустой = дефолт: data-hoist, basic). Неизвестный id → ошибка манифеста.
pub fn installers_from_ids(ids: &[String]) -> Result<Vec<Box<dyn ModInstaller>>, CoreError> {
    let resolved: Vec<String> = if ids.is_empty() {
        vec![
            INSTALLER_ID_DATA_HOIST.to_string(),
            INSTALLER_ID_BASIC.to_string(),
        ]
    } else {
        ids.to_vec()
    };
    let mut out = Vec::with_capacity(resolved.len());
    for id in resolved {
        out.push(boxed_installer(&id).ok_or(CoreError::PluginInvalidManifest)?);
    }
    Ok(out)
}

/// Run the chain: first installer with `test_supported` runs and returns.
pub fn run_installer_chain(
    staging_mod_dir: &Path,
    relative_paths: &[String],
    chain: &[Box<dyn ModInstaller>],
) -> Result<&'static str, CoreError> {
    let mut ordered: Vec<_> = chain.iter().collect();
    ordered.sort_by(|a, b| b.priority().cmp(&a.priority()));

    for inst in ordered {
        if inst.test_supported(relative_paths) {
            inst.install(staging_mod_dir)?;
            return Ok(inst.id());
        }
    }
    Err(CoreError::InvalidInput)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn basic_always_matches() {
        let b = BasicInstaller;
        assert!(b.test_supported(&[]));
        assert!(b.test_supported(&["a/b.txt".into()]));
    }

    #[test]
    fn data_hoist_matches_only_data_root() {
        let d = DataHoistInstaller;
        assert!(d.test_supported(&["Data/foo.esp".into()]));
        assert!(!d.test_supported(&["readme.txt".into(), "Data/foo.esp".into()]));
    }

    #[test]
    fn chain_picks_data_hoist() {
        let tmp = std::env::temp_dir().join(format!("pantheon-install-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(tmp.join("Data")).unwrap();
        fs::write(tmp.join("Data").join("x.txt"), b"hi").unwrap();
        let paths = vec!["Data/x.txt".into()];
        let chain = installers_from_ids(&[]).unwrap();
        let id = run_installer_chain(&tmp, &paths, &chain).unwrap();
        assert_eq!(id, "data-hoist");
        assert!(tmp.join("x.txt").exists());
        assert!(!tmp.join("Data").exists());
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn chain_falls_back_to_basic() {
        let tmp = PathBuf::from(std::env::temp_dir()).join(format!("pantheon-basic-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&tmp).unwrap();
        fs::write(tmp.join("root.txt"), b"x").unwrap();
        let paths = vec!["root.txt".into()];
        let chain = installers_from_ids(&[]).unwrap();
        let id = run_installer_chain(&tmp, &paths, &chain).unwrap();
        assert_eq!(id, "basic");
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn installers_from_ids_rejects_unknown() {
        assert!(matches!(
            installers_from_ids(&["unknown-installer".into()]),
            Err(CoreError::PluginInvalidManifest)
        ));
    }

    #[test]
    fn installers_basic_only_skips_hoist() {
        let tmp = std::env::temp_dir().join(format!("pantheon-basic-only-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(tmp.join("Data")).unwrap();
        fs::write(tmp.join("Data").join("x.txt"), b"hi").unwrap();
        let paths = vec!["Data/x.txt".into()];
        let chain = installers_from_ids(&[INSTALLER_ID_BASIC.into()]).unwrap();
        let id = run_installer_chain(&tmp, &paths, &chain).unwrap();
        assert_eq!(id, "basic");
        assert!(tmp.join("Data").join("x.txt").exists());
        let _ = fs::remove_dir_all(&tmp);
    }
}
