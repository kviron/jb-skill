use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::{errors::CoreError, installers};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub api_version: String,
    pub game_id: String,
    pub permissions: Vec<String>,
    /// Id инсталлеров ядра в порядке участия (см. `core::installers`). Пустой массив = `data-hoist` + `basic`.
    #[serde(default)]
    pub installers: Vec<String>,
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectGameResult {
    pub detected: bool,
    pub install_path: Option<String>,
    pub mod_path: Option<String>,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseModResult {
    pub mod_type: String,
    pub warnings: Vec<String>,
    pub required_tools: Vec<String>,
    pub layout_summary: String,
    pub supported: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallAction {
    pub kind: String,
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallPlan {
    pub actions: Vec<InstallAction>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployEntry {
    pub target_path: String,
    pub winner_mod_id: String,
    pub source_path: String,
    pub strategy: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployPlan {
    pub entries: Vec<DeployEntry>,
    pub conflict_candidates: Vec<String>,
    pub post_deploy_hooks: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub ok: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
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
    for id in &manifest.installers {
        if !installers::installer_id_is_valid(id) {
            return Err(CoreError::PluginInvalidManifest);
        }
    }
    Ok(())
}

fn major(version: &str) -> Option<u64> {
    version.split('.').next()?.parse::<u64>().ok()
}

pub fn plugin_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("plugins")
}

pub fn run_smoke_test(manifest: &PluginManifest, archive_path: &str) -> Result<SmokeResult, CoreError> {
    let detect_game_ok = detect_game(manifest, vec!["C:/Games/PilotGame".into()])?.detected;
    let parse_mod_ok = parse_mod(manifest, archive_path)?.supported;
    plan_install(manifest, archive_path, "mod_sample")?;
    let plan_install_ok = true;
    let sample_files = vec![("Data/pilot.esp".to_string(), "sha256:smoke".to_string())];
    let plan_deploy_ok = plan_deploy_from_staging("mod_sample", &sample_files).is_ok();
    let validate_ok = validate(manifest, archive_path)?.ok;

    Ok(SmokeResult {
        plugin_id: manifest.id.clone(),
        detect_game_ok,
        parse_mod_ok,
        plan_install_ok,
        plan_deploy_ok,
        validate_ok,
    })
}

pub fn detect_game(manifest: &PluginManifest, candidates: Vec<String>) -> Result<DetectGameResult, CoreError> {
    run_hook_with_timeout(Duration::from_secs(2), || {
        if manifest.game_id.is_empty() {
            return Err(CoreError::PluginValidationFailed);
        }
        let path = candidates
            .into_iter()
            .find(|candidate| candidate.to_lowercase().contains("pilotgame"));
        let has_path = path.is_some();
        Ok(DetectGameResult {
            detected: has_path,
            install_path: path.clone(),
            mod_path: path.map(|item| format!("{item}/Mods")),
            issues: if has_path {
                vec![]
            } else {
                vec!["GAME_NOT_FOUND".to_string()]
            },
        })
    })
}

pub fn parse_mod(_manifest: &PluginManifest, archive_path: &str) -> Result<ParseModResult, CoreError> {
    run_hook_with_timeout(Duration::from_secs(10), || {
        let supported = crate::core::archive_ext::path_looks_like_mod_archive(std::path::Path::new(archive_path));
        Ok(ParseModResult {
            mod_type: if supported {
                "loose-files".to_string()
            } else {
                "unsupported".to_string()
            },
            warnings: if supported {
                vec![]
            } else {
                vec!["UNSUPPORTED_ARCHIVE".to_string()]
            },
            required_tools: vec![],
            layout_summary: if supported {
                "archive-root".to_string()
            } else {
                "unknown".to_string()
            },
            supported,
        })
    })
}

/// Plugin-level install steps (paths relative to staging mod root). Empty = rely on core installer chain only.
pub fn plan_install(_manifest: &PluginManifest, archive_path: &str, _mod_id: &str) -> Result<InstallPlan, CoreError> {
    run_hook_with_timeout(Duration::from_secs(10), || {
        if archive_path.contains("..") {
            return Err(CoreError::PluginInvalidPlan);
        }
        Ok(InstallPlan { actions: vec![] })
    })
}

/// Build deploy plan from the **final** staging file list (after extract + installer chain + `plan_install` actions).
pub fn plan_deploy_from_staging(mod_id: &str, files: &[(String, String)]) -> Result<DeployPlan, CoreError> {
    run_hook_with_timeout(Duration::from_secs(10), || {
        if files.is_empty() {
            return Err(CoreError::PluginInvalidPlan);
        }
        let mut entries = Vec::new();
        for (rel, _) in files {
            if rel.contains("..") {
                return Err(CoreError::PluginInvalidPlan);
            }
            let rel_norm = rel.replace('\\', "/");
            entries.push(DeployEntry {
                target_path: rel_norm.clone(),
                winner_mod_id: mod_id.to_string(),
                source_path: format!("<staging>/{mod_id}/{rel_norm}"),
                strategy: "copy".to_string(),
            });
        }
        Ok(DeployPlan {
            entries,
            conflict_candidates: vec![],
            post_deploy_hooks: vec![],
        })
    })
}

pub fn validate(_manifest: &PluginManifest, archive_path: &str) -> Result<ValidationResult, CoreError> {
    run_hook_with_timeout(Duration::from_secs(5), || {
        if archive_path.contains("..") {
            return Ok(ValidationResult {
                ok: false,
                errors: vec!["PATH_TRAVERSAL_DETECTED".to_string()],
                warnings: vec![],
            });
        }
        Ok(ValidationResult {
            ok: true,
            errors: vec![],
            warnings: vec![],
        })
    })
}

pub fn validate_deploy_plan(plan: &DeployPlan) -> Result<(), CoreError> {
    if plan.entries.is_empty() {
        return Err(CoreError::PluginInvalidPlan);
    }
    let has_unsafe_paths = plan
        .entries
        .iter()
        .any(|entry| entry.target_path.contains("..") || entry.source_path.contains(".."));
    if has_unsafe_paths {
        return Err(CoreError::PluginInvalidPlan);
    }
    let _ = json!(plan);
    Ok(())
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
    fn rejects_unknown_installer_id() {
        let manifest = PluginManifest {
            id: "com.jb.test".into(),
            name: "Test".into(),
            version: "0.1.0".into(),
            api_version: "1.0.0".into(),
            game_id: "pilot-game".into(),
            permissions: vec!["game.read".into(), "mods.install".into(), "deploy.plan".into()],
            installers: vec!["not-a-real-installer".into()],
        };
        assert!(matches!(
            validate_manifest(&manifest),
            Err(CoreError::PluginInvalidManifest)
        ));
    }

    #[test]
    fn validates_manifest_permissions() {
        let manifest = PluginManifest {
            id: "com.jb.test".into(),
            name: "Test".into(),
            version: "0.1.0".into(),
            api_version: "1.0.0".into(),
            game_id: "pilot-game".into(),
            permissions: vec!["game.read".into(), "mods.install".into()],
            installers: vec![],
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
            installers: vec![],
        };
        let result = run_smoke_test(&manifest, "../unsafe.zip");
        assert!(matches!(result, Err(CoreError::PluginInvalidPlan)));
    }
}
