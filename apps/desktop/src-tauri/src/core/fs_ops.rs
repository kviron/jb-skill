use std::fs::{self, File};
use std::io::copy;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use rusqlite::Connection;
use sha2::{Digest, Sha256};
use unrar::Archive;
use zip::ZipArchive;

use crate::core::archive_ext::{mod_archive_format, ModArchiveFormat};
use crate::core::errors::CoreError;
use crate::core::plugins::InstallAction;

/// Extract a `.zip` archive into `staging_mod_dir` (e.g. `app_data/pantheon/staging/<mod_id>`).
/// Returns relative paths (posix) and checksums for inserted `mod_files` rows.
pub fn extract_zip_to_staging(archive_path: &Path, staging_mod_dir: &Path) -> Result<Vec<(String, String)>, CoreError> {
    fs::create_dir_all(staging_mod_dir)?;
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file).map_err(|e| CoreError::Io(e.into()))?;
    let mut out = Vec::new();

    for i in 0..archive.len() {
        let mut zf = archive.by_index(i).map_err(|e| CoreError::Io(e.into()))?;
        let name = zf.name();
        if name.ends_with('/') {
            let outpath = staging_mod_dir.join(name.trim_end_matches('/'));
            fs::create_dir_all(&outpath)?;
            continue;
        }
        let Some(enclosed) = zf.enclosed_name() else {
            continue;
        };
        let rel = enclosed.to_string_lossy().replace('\\', "/");
        if rel.is_empty() || rel.starts_with("__MACOSX/") {
            continue;
        }
        let outpath = staging_mod_dir.join(enclosed);
        if let Some(parent) = outpath.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut outfile = fs::File::create(&outpath)?;
        copy(&mut zf, &mut outfile)?;
        let checksum = hash_file_sha256(&outpath)?;
        out.push((rel, checksum));
    }

    if out.is_empty() {
        return Err(CoreError::InvalidInput);
    }
    Ok(out)
}

/// Extract a supported mod archive into staging; returns relative paths (posix) and checksums.
pub fn extract_archive_to_staging(archive_path: &Path, staging_mod_dir: &Path) -> Result<Vec<(String, String)>, CoreError> {
    match mod_archive_format(archive_path) {
        Some(ModArchiveFormat::Zip) => extract_zip_to_staging(archive_path, staging_mod_dir),
        Some(ModArchiveFormat::SevenZ) => extract_7z_to_staging(archive_path, staging_mod_dir),
        Some(ModArchiveFormat::Rar) => extract_rar_to_staging(archive_path, staging_mod_dir),
        None => Err(CoreError::InvalidInput),
    }
}

fn unrar_err(e: impl std::fmt::Display) -> CoreError {
    CoreError::Io(std::io::Error::new(ErrorKind::Other, format!("rar: {e}")))
}

/// Extract RAR via `unrar` crate (UnRAR DLL), then hash files like zip/7z.
pub fn extract_rar_to_staging(archive_path: &Path, staging_mod_dir: &Path) -> Result<Vec<(String, String)>, CoreError> {
    fs::create_dir_all(staging_mod_dir)?;
    let mut archive = Archive::new(archive_path)
        .open_for_processing()
        .map_err(unrar_err)?;
    while let Some(arc) = archive.read_header().map_err(unrar_err)? {
        let entry = arc.entry();
        if entry.is_encrypted() {
            return Err(CoreError::InvalidInput);
        }
        if entry.is_directory() {
            archive = arc.skip().map_err(unrar_err)?;
            continue;
        }
        let rel = entry.filename.to_string_lossy().replace('\\', "/");
        if rel.is_empty() || rel.starts_with("__MACOSX/") {
            archive = arc.skip().map_err(unrar_err)?;
            continue;
        }
        let dest = staging_mod_dir.join(&entry.filename);
        if let Some(p) = dest.parent() {
            fs::create_dir_all(p).map_err(|e| CoreError::Io(e.into()))?;
        }
        archive = arc.extract_to(&dest).map_err(unrar_err)?;
    }
    walk_staging_with_hashes(staging_mod_dir)
}

/// Extract 7z via `sevenz-rust`, then hash all files (same layout contract as zip).
pub fn extract_7z_to_staging(archive_path: &Path, staging_mod_dir: &Path) -> Result<Vec<(String, String)>, CoreError> {
    fs::create_dir_all(staging_mod_dir)?;
    sevenz_rust::decompress_file(archive_path, staging_mod_dir).map_err(|e| {
        CoreError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("7z decompress: {e}"),
        ))
    })?;
    walk_staging_with_hashes(staging_mod_dir)
}

/// All files under `staging_mod_dir` with posix-relative paths and SHA-256 checksums.
pub fn walk_staging_with_hashes(staging_mod_dir: &Path) -> Result<Vec<(String, String)>, CoreError> {
    let mut out = Vec::new();
    walk_dir_files(staging_mod_dir, staging_mod_dir, &mut out)?;
    if out.is_empty() {
        return Err(CoreError::InvalidInput);
    }
    Ok(out)
}

fn walk_dir_files(base: &Path, current: &Path, out: &mut Vec<(String, String)>) -> Result<(), CoreError> {
    for entry in fs::read_dir(current).map_err(|e| CoreError::Io(e.into()))? {
        let entry = entry.map_err(|e| CoreError::Io(e.into()))?;
        let path = entry.path();
        let meta = entry.metadata().map_err(|e| CoreError::Io(e.into()))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if meta.is_dir() {
            if name == "__MACOSX" {
                continue;
            }
            walk_dir_files(base, &path, out)?;
        } else if meta.is_file() {
            let rel = path.strip_prefix(base).map_err(|_| CoreError::InvalidInput)?;
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if rel_str.starts_with("__MACOSX/") {
                continue;
            }
            let checksum = hash_file_sha256(&path)?;
            out.push((rel_str, checksum));
        }
    }
    Ok(())
}

/// Apply plugin [`InstallAction`]s inside `staging_mod_dir` only (paths must not contain `..`).
pub fn apply_install_plan_to_staging(staging_mod_dir: &Path, actions: &[InstallAction]) -> Result<(), CoreError> {
    for a in actions {
        let kind = a.kind.to_lowercase();
        if kind != "copy" && kind != "move" {
            return Err(CoreError::PluginInvalidPlan);
        }
        let src = safe_join_under(staging_mod_dir, &a.source)?;
        let dst = safe_join_under(staging_mod_dir, &a.destination)?;
        if kind == "copy" {
            if let Some(p) = dst.parent() {
                fs::create_dir_all(p).map_err(|e| CoreError::Io(e.into()))?;
            }
            fs::copy(&src, &dst).map_err(|e| CoreError::Io(e.into()))?;
        } else {
            if let Some(p) = dst.parent() {
                fs::create_dir_all(p).map_err(|e| CoreError::Io(e.into()))?;
            }
            fs::rename(&src, &dst).map_err(|e| CoreError::Io(e.into()))?;
        }
    }
    Ok(())
}

fn safe_join_under(base: &Path, rel: &str) -> Result<PathBuf, CoreError> {
    let mut p = base.to_path_buf();
    for part in rel.replace('\\', "/").split('/').filter(|s| !s.is_empty()) {
        if part == ".." {
            return Err(CoreError::PluginInvalidPlan);
        }
        p.push(part);
    }
    Ok(p)
}

fn hash_file_sha256(path: &Path) -> Result<String, CoreError> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

pub fn remove_staging_dir(staging_root: &Path, mod_id: &str) -> Result<(), CoreError> {
    let dir = staging_root.join(mod_id);
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    Ok(())
}

/// Copy winners from staging into `games.mod_path` according to current deploy manifest for `profile_id`.
pub fn apply_deploy_to_mod_path(conn: &Connection, profile_id: &str, staging_root: &Path) -> Result<(), CoreError> {
    let mod_path: String = conn.query_row(
        "SELECT g.mod_path FROM profiles p JOIN games g ON g.id = p.game_id WHERE p.id = ?1",
        [profile_id],
        |row| row.get(0),
    )?;

    let mod_root = PathBuf::from(mod_path);
    fs::create_dir_all(&mod_root)?;

    let manifest_json: String = conn.query_row(
        "SELECT manifest_json FROM deploy_state WHERE profile_id = ?1 AND is_current = 1 ORDER BY created_at DESC LIMIT 1",
        [profile_id],
        |row| row.get(0),
    )?;

    let v: serde_json::Value =
        serde_json::from_str(&manifest_json).map_err(|_| CoreError::InvalidInput)?;
    let entries = v["entries"].as_array().ok_or(CoreError::InvalidInput)?;

    for e in entries {
        let target = e["targetPath"].as_str().ok_or(CoreError::InvalidInput)?;
        let winner = e["winnerModId"].as_str().ok_or(CoreError::InvalidInput)?;
        let rel = target.replace('\\', "/");
        let src = staging_root.join(winner).join(&rel);
        let dst = mod_root.join(&rel);
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&src, &dst)?;
    }
    Ok(())
}

/// Best-effort remove deployed files for given relative paths under `mod_path` root.
pub fn remove_deployed_files(mod_root: &Path, relative_paths: &[String]) {
    for rel in relative_paths {
        let p = mod_root.join(rel);
        if p.is_file() {
            let _ = fs::remove_file(&p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn apply_install_rejects_parent_segments() {
        let tmp = std::env::temp_dir().join(format!("pantheon-fs-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&tmp).unwrap();
        let a = InstallAction {
            kind: "copy".into(),
            source: "../outside.txt".into(),
            destination: "a.txt".into(),
        };
        assert!(apply_install_plan_to_staging(&tmp, &[a]).is_err());
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn apply_install_copy_within_staging() {
        let tmp = std::env::temp_dir().join(format!("pantheon-fs2-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&tmp.join("sub")).unwrap();
        let mut f = File::create(tmp.join("sub").join("s.txt")).unwrap();
        f.write_all(b"x").unwrap();
        let a = InstallAction {
            kind: "copy".into(),
            source: "sub/s.txt".into(),
            destination: "t.txt".into(),
        };
        apply_install_plan_to_staging(&tmp, &[a]).unwrap();
        assert!(tmp.join("t.txt").exists());
        let _ = fs::remove_dir_all(&tmp);
    }
}
