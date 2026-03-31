use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

use crate::core::errors::CoreError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub api_version: String,
    pub game_id: String,
    pub permissions: Vec<String>,
}

pub const CORE_PLUGIN_API_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmokeResult {
    pub plugin_id: String,
    pub detect_game_ok: bool,
    pub parse_mod_ok: bool,
    pub plan_install_ok: bool,
    pub plan_deploy_ok: bool,
    pub validate_ok: bool,
}

pub fn load_manifest(path: &Path) -> Result<PluginManifest, CoreError> {
    let content = fs::read_to_string(path).map_err(|_| CoreError::PluginInvalidManifest)?;
    let manifest: PluginManifest =
        serde_json::from_str(&content).map_err(|_| CoreError::PluginInvalidManifest)?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

pub fn validate_manifest(manifest: &PluginManifest) -> Result<(), CoreError> {
    if manifest.id.is_empty()
        || manifest.name.is_empty()
        || manifest.version.is_empty()
        || manifest.api_version.is_empty()
        || manifest.game_id.is_empty()
    {
        return Err(CoreError::PluginInvalidManifest);
    }
    if major(&manifest.api_version) != major(CORE_PLUGIN_API_VERSION) {
        return Err(CoreError::PluginApiIncompatible);
    }

    let allowed = ["game.read", "mods.install", "deploy.plan", "mods.validate"];
    if manifest
        .permissions
        .iter()
        .any(|perm| !allowed.contains(&perm.as_str()))
    {
        return Err(CoreError::PluginPermissionDenied);
    }
    Ok(())
}

fn major(version: &str) -> Option<u64> {
    version.split('.').next()?.parse::<u64>().ok()
}

pub fn plugin_root() -> PathBuf {
    PathBuf::from("../..").join("plugins")
}

pub fn run_smoke_test(manifest: &PluginManifest, archive_path: &str) -> Result<SmokeResult, CoreError> {
    let detect_game_ok = run_hook_with_timeout(Duration::from_secs(2), || {
        if manifest.game_id.is_empty() {
            return Err(CoreError::PluginValidationFailed);
        }
        Ok(true)
    })?;
    let parse_mod_ok = run_hook_with_timeout(Duration::from_secs(10), || {
        if !(archive_path.ends_with(".zip") || archive_path.ends_with(".7z")) {
            return Err(CoreError::PluginValidationFailed);
        }
        Ok(true)
    })?;
    let plan_install_ok = run_hook_with_timeout(Duration::from_secs(10), || Ok(true))?;
    let plan_deploy_ok = run_hook_with_timeout(Duration::from_secs(10), || Ok(true))?;
    let validate_ok = run_hook_with_timeout(Duration::from_secs(5), || {
        if archive_path.contains("..") {
            return Err(CoreError::PluginValidationFailed);
        }
        Ok(true)
    })?;

    Ok(SmokeResult {
        plugin_id: manifest.id.clone(),
        detect_game_ok,
        parse_mod_ok,
        plan_install_ok,
        plan_deploy_ok,
        validate_ok,
    })
}

fn run_hook_with_timeout<T, F>(timeout: Duration, f: F) -> Result<T, CoreError>
where
    F: FnOnce() -> Result<T, CoreError>,
{
    let started = Instant::now();
    let value = f()?;
    if started.elapsed() > timeout {
        return Err(CoreError::PluginTimeout);
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_manifest_permissions() {
        let manifest = PluginManifest {
            id: "com.jb.test".into(),
            name: "Test".into(),
            version: "0.1.0".into(),
            api_version: "1.0.0".into(),
            game_id: "pilot-game".into(),
            permissions: vec!["game.read".into(), "mods.install".into()],
        };
        let result = validate_manifest(&manifest);
        assert!(result.is_ok());
    }

    #[test]
    fn smoke_rejects_unsafe_path() {
        let manifest = PluginManifest {
            id: "com.jb.test".into(),
            name: "Test".into(),
            version: "0.1.0".into(),
            api_version: "1.0.0".into(),
            game_id: "pilot-game".into(),
            permissions: vec!["game.read".into(), "mods.install".into(), "deploy.plan".into()],
        };
        let result = run_smoke_test(&manifest, "../unsafe.zip");
        assert!(matches!(result, Err(CoreError::PluginValidationFailed)));
    }
}
