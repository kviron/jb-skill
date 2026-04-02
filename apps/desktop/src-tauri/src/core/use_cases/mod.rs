use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

use crate::core::{
    contracts::{
        ConflictRecord, InstallResult, ModRecord, OperationResult, PrepareModInstallResult, SwitchProfileResult,
    },
    errors::CoreError,
    fomod::{
        apply_fomod_selections, detect_scripted_installer, find_module_config_relative, module_config_abs_path,
        parse_module_config_str, FomodSelections,
    },
    fs_ops::{
        apply_deploy_to_mod_path, apply_install_plan_to_staging, extract_archive_to_staging, remove_deployed_files,
        remove_staging_dir, walk_staging_with_hashes,
    },
    install_session::{InstallSession, InstallSessionKind},
    installers::{installers_from_ids, run_installer_chain},
    plugins,
    tx,
};

fn now() -> String {
    Utc::now().to_rfc3339()
}

fn game_pilot_manifest_path() -> PathBuf {
    plugins::plugin_root().join("game-pilot").join("manifest.json")
}

/// `true` if the tree is a valid XML FOMOD that must go through the wizard. Scripted installers return `Err`.
fn staging_requires_fomod_wizard(staging_dir: &Path) -> Result<bool, CoreError> {
    if let Some(rel) = find_module_config_relative(staging_dir)? {
        let path = module_config_abs_path(staging_dir, &rel);
        let raw = fs::read_to_string(&path).map_err(|e| CoreError::Io(e.into()))?;
        if detect_scripted_installer(&raw) {
            return Err(CoreError::FomodScriptedNotSupported);
        }
        parse_module_config_str(&raw)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn finish_install_from_staging(
    conn: &mut Connection,
    game_id: &str,
    profile_id: &str,
    archive_path: &str,
    staging_mod_dir: &Path,
    mod_id: &str,
    staging_root: &Path,
) -> Result<InstallResult, CoreError> {
    let manifest = plugins::load_manifest(&game_pilot_manifest_path())?;
    let archive_path_buf = PathBuf::from(archive_path);
    let parsed_name = archive_path_buf
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("New Mod")
        .to_string();

    let tx_sql = conn.transaction()?;
    let operation_id = tx::start_transaction(&tx_sql, "install", Some(profile_id))?;

    let profile_exists: Option<String> = tx_sql
        .query_row(
            "SELECT id FROM profiles WHERE id = ?1 AND game_id = ?2",
            params![profile_id, game_id],
            |row| row.get(0),
        )
        .optional()?;
    if profile_exists.is_none() {
        return Err(CoreError::ProfileNotFound);
    }

    let parse_started = Instant::now();
    let parsed = plugins::parse_mod(&manifest, archive_path)?;
    tx::add_step(
        &tx_sql,
        &operation_id,
        1,
        "parse_mod",
        &serde_json::json!({ "archivePath": archive_path }).to_string(),
        None,
        "done",
        parse_started.elapsed().as_millis() as i64,
        None,
    )?;
    if !parsed.supported {
        return Err(CoreError::PluginValidationFailed);
    }

    let validate_started = Instant::now();
    let validation = plugins::validate(&manifest, archive_path)?;
    tx::add_step(
        &tx_sql,
        &operation_id,
        2,
        "validate",
        &serde_json::json!({ "archivePath": archive_path }).to_string(),
        Some(r#"{"compensation":"abort_transaction"}"#),
        if validation.ok { "done" } else { "failed" },
        validate_started.elapsed().as_millis() as i64,
        if validation.ok { None } else { Some("PLUGIN_VALIDATION_FAILED") },
    )?;
    if !validation.ok {
        return Err(CoreError::PluginValidationFailed);
    }

    let rel_paths: Vec<String> = walk_staging_with_hashes(staging_mod_dir)?
        .into_iter()
        .map(|(r, _)| r)
        .collect();
    let chain = installers_from_ids(&manifest.installers)?;
    let _installer_id = run_installer_chain(staging_mod_dir, &rel_paths, &chain)?;

    let install_plan = plugins::plan_install(&manifest, archive_path, mod_id)?;
    apply_install_plan_to_staging(staging_mod_dir, &install_plan.actions)?;

    let final_files = walk_staging_with_hashes(staging_mod_dir)?;

    let max_priority: i64 = tx_sql.query_row(
        "SELECT COALESCE(MAX(pm.priority), 0) FROM profile_mods pm
         INNER JOIN mods m ON m.id = pm.mod_id
         WHERE m.game_id = ?1 AND pm.profile_id = ?2",
        params![game_id, profile_id],
        |row| row.get(0),
    )?;

    tx_sql.execute(
        "INSERT INTO mods (id, game_id, name, version, archive_path, installed_at)
         VALUES (?1, ?2, ?3, '1.0.0', ?4, ?5)",
        params![mod_id, game_id, parsed_name, archive_path, now()],
    )?;

    tx_sql.execute(
        "INSERT INTO profile_mods (profile_id, mod_id, enabled, priority)
         VALUES (?1, ?2, 1, ?3)",
        params![profile_id, mod_id, max_priority + 1],
    )?;

    for (rel, checksum) in &final_files {
        tx_sql.execute(
            "INSERT INTO mod_files (id, mod_id, relative_path, checksum, size_bytes) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                Uuid::new_v4().to_string(),
                mod_id,
                rel,
                checksum,
                0_i64
            ],
        )?;
    }

    let install_plan_started = Instant::now();
    tx::add_step(
        &tx_sql,
        &operation_id,
        3,
        "plan_install",
        &serde_json::to_string(&install_plan.actions).unwrap_or_else(|_| "[]".to_string()),
        Some(r#"{"compensation":"remove_installed_files"}"#),
        "done",
        install_plan_started.elapsed().as_millis() as i64,
        None,
    )?;
    let deploy_plan_started = Instant::now();
    let deploy_plan = plugins::plan_deploy_from_staging(mod_id, &final_files)?;
    plugins::validate_deploy_plan(&deploy_plan)?;
    tx::add_step(
        &tx_sql,
        &operation_id,
        4,
        "plan_deploy",
        &serde_json::to_string(&deploy_plan.entries).unwrap_or_else(|_| "[]".to_string()),
        Some(r#"{"compensation":"restore_previous_deploy_manifest"}"#),
        "done",
        deploy_plan_started.elapsed().as_millis() as i64,
        None,
    )?;
    let _deploy_state_id = persist_deploy_state(&tx_sql, profile_id, staging_root)?;
    recalc_conflicts(&tx_sql, profile_id)?;
    tx::finish_transaction(&tx_sql, &operation_id, "committed")?;
    tx_sql.commit()?;

    apply_deploy_to_mod_path(conn, profile_id, staging_root)?;

    Ok(InstallResult {
        operation_id,
        mod_id: mod_id.to_string(),
        warnings: vec![],
    })
}

/// Extract archive into a session folder; detect XML FOMOD vs plain.
pub fn prepare_mod_install(
    staging_root: &Path,
    archive_path: &str,
    sessions: &Mutex<HashMap<String, InstallSession>>,
) -> Result<PrepareModInstallResult, CoreError> {
    if archive_path.trim().is_empty() {
        return Err(CoreError::InvalidInput);
    }
    let archive_path_buf = PathBuf::from(archive_path);
    if !archive_path_buf.is_file() {
        return Err(CoreError::InvalidInput);
    }

    let session_id = Uuid::new_v4().to_string();
    let sessions_dir = staging_root.join("_fomod_sessions");
    fs::create_dir_all(&sessions_dir).map_err(|e| CoreError::Io(e.into()))?;
    let extract_root = sessions_dir.join(&session_id);
    extract_archive_to_staging(&archive_path_buf, &extract_root)?;

    if let Some(rel) = find_module_config_relative(&extract_root)? {
        let path = module_config_abs_path(&extract_root, &rel);
        let raw = fs::read_to_string(&path).map_err(|e| CoreError::Io(e.into()))?;
        if detect_scripted_installer(&raw) {
            let _ = fs::remove_dir_all(&extract_root);
            return Err(CoreError::FomodScriptedNotSupported);
        }
        let parsed = parse_module_config_str(&raw)?;
        let wizard = parsed.to_wizard_payload();
        let mut guard = sessions.lock().map_err(|_| CoreError::InvalidInput)?;
        guard.insert(
            session_id.clone(),
            InstallSession {
                extract_root,
                archive_path: archive_path.to_string(),
                kind: InstallSessionKind::Fomod { parsed },
            },
        );
        return Ok(PrepareModInstallResult {
            session_id,
            kind: "fomod".into(),
            wizard: Some(wizard),
        });
    }

    let mut guard = sessions.lock().map_err(|_| CoreError::InvalidInput)?;
    guard.insert(
        session_id.clone(),
        InstallSession {
            extract_root,
            archive_path: archive_path.to_string(),
            kind: InstallSessionKind::Plain,
        },
    );
    Ok(PrepareModInstallResult {
        session_id,
        kind: "plain".into(),
        wizard: None,
    })
}

pub fn cancel_mod_install_session(
    staging_root: &Path,
    session_id: &str,
    sessions: &Mutex<HashMap<String, InstallSession>>,
) -> Result<(), CoreError> {
    let removed = {
        let mut guard = sessions.lock().map_err(|_| CoreError::InvalidInput)?;
        guard.remove(session_id)
    };
    if let Some(s) = removed {
        if s.extract_root.exists() {
            let _ = fs::remove_dir_all(&s.extract_root);
        }
        Ok(())
    } else {
        let orphan = staging_root.join("_fomod_sessions").join(session_id);
        if orphan.exists() {
            let _ = fs::remove_dir_all(&orphan);
            Ok(())
        } else {
            Err(CoreError::InstallSessionNotFound)
        }
    }
}

pub fn finalize_mod_install(
    conn: &mut Connection,
    game_id: &str,
    profile_id: &str,
    session_id: &str,
    fomod_selections: Option<FomodSelections>,
    staging_root: &Path,
    sessions: &Mutex<HashMap<String, InstallSession>>,
) -> Result<InstallResult, CoreError> {
    let session = {
        let mut guard = sessions.lock().map_err(|_| CoreError::InvalidInput)?;
        guard.remove(session_id).ok_or(CoreError::InstallSessionNotFound)?
    };

    match &session.kind {
        InstallSessionKind::Plain if fomod_selections.is_some() => {
            let mut guard = sessions.lock().map_err(|_| CoreError::InvalidInput)?;
            guard.insert(session_id.to_string(), session);
            return Err(CoreError::InvalidInput);
        }
        InstallSessionKind::Fomod { .. } if fomod_selections.is_none() => {
            let mut guard = sessions.lock().map_err(|_| CoreError::InvalidInput)?;
            guard.insert(session_id.to_string(), session);
            return Err(CoreError::FomodInvalidSelection);
        }
        _ => {}
    }

    let mod_id = Uuid::new_v4().to_string();
    let staging_mod_dir = staging_root.join(&mod_id);

    let InstallSession {
        extract_root,
        archive_path,
        kind,
    } = session;

    fs::rename(&extract_root, &staging_mod_dir).map_err(|e| CoreError::Io(e.into()))?;

    if let InstallSessionKind::Fomod { parsed } = kind {
        let sel = fomod_selections.ok_or(CoreError::FomodInvalidSelection)?;
        apply_fomod_selections(&staging_mod_dir, &parsed, &sel)?;
    }

    finish_install_from_staging(
        conn,
        game_id,
        profile_id,
        &archive_path,
        &staging_mod_dir,
        &mod_id,
        staging_root,
    )
}

pub fn list_mods(conn: &Connection, game_id: &str, profile_id: &str) -> Result<Vec<ModRecord>, CoreError> {
    let mut stmt = conn.prepare(
        "SELECT m.id, m.game_id, m.name, m.version, m.archive_path, pm.enabled, pm.priority, m.installed_at
         FROM mods m
         INNER JOIN profile_mods pm ON pm.mod_id = m.id AND pm.profile_id = ?2
         WHERE m.game_id = ?1
         ORDER BY pm.priority DESC, m.installed_at DESC",
    )?;

    let rows = stmt.query_map(params![game_id, profile_id], |row| {
        Ok(ModRecord {
            id: row.get(0)?,
            game_id: row.get(1)?,
            name: row.get(2)?,
            version: row.get(3)?,
            archive_path: row.get(4)?,
            enabled: row.get::<_, i64>(5)? == 1,
            priority: row.get(6)?,
            installed_at: row.get(7)?,
        })
    })?;

    let mut mods = Vec::new();
    for row in rows {
        mods.push(row?);
    }
    Ok(mods)
}

pub fn get_conflicts(conn: &Connection, profile_id: &str) -> Result<Vec<ConflictRecord>, CoreError> {
    let mut stmt = conn.prepare(
        "SELECT id, profile_id, target_path, winner_mod_id, loser_mod_ids_json, resolved_by, updated_at
         FROM conflicts WHERE profile_id = ?1 ORDER BY updated_at DESC",
    )?;
    let rows = stmt.query_map([profile_id], |row| {
        Ok(ConflictRecord {
            id: row.get(0)?,
            profile_id: row.get(1)?,
            target_path: row.get(2)?,
            winner_mod_id: row.get(3)?,
            loser_mod_ids_json: row.get(4)?,
            resolved_by: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;

    let mut conflicts = Vec::new();
    for row in rows {
        conflicts.push(row?);
    }
    Ok(conflicts)
}

/// One-shot install for **non-FOMOD** archives. XML FOMOD archives return [`CoreError::FomodRequiresWizard`].
pub fn install_mod_from_archive(
    conn: &mut Connection,
    game_id: &str,
    profile_id: &str,
    archive_path: &str,
    staging_root: &Path,
) -> Result<InstallResult, CoreError> {
    if archive_path.trim().is_empty() {
        return Err(CoreError::InvalidInput);
    }
    let archive_path_buf = PathBuf::from(archive_path);
    if !archive_path_buf.exists() || !archive_path_buf.is_file() {
        return Err(CoreError::InvalidInput);
    }

    let mod_id = Uuid::new_v4().to_string();
    let staging_mod_dir = staging_root.join(&mod_id);

    let sessions_dir = staging_root.join("_fomod_sessions");
    fs::create_dir_all(&sessions_dir).map_err(|e| CoreError::Io(e.into()))?;
    let temp = sessions_dir.join(format!("legacy-{}", Uuid::new_v4()));
    extract_archive_to_staging(&archive_path_buf, &temp)?;

    if staging_requires_fomod_wizard(&temp)? {
        let _ = fs::remove_dir_all(&temp);
        return Err(CoreError::FomodRequiresWizard);
    }

    fs::rename(&temp, &staging_mod_dir).map_err(|e| CoreError::Io(e.into()))?;

    finish_install_from_staging(
        conn,
        game_id,
        profile_id,
        archive_path,
        &staging_mod_dir,
        &mod_id,
        staging_root,
    )
}

pub fn set_mod_enabled(
    conn: &mut Connection,
    profile_id: &str,
    mod_id: &str,
    enabled: bool,
    staging_root: &Path,
) -> Result<OperationResult, CoreError> {
    let tx_sql = conn.transaction()?;
    let operation_id = tx::start_transaction(&tx_sql, "set_enabled", Some(profile_id))?;
    let step_started = Instant::now();
    tx_sql.execute(
        "UPDATE profile_mods SET enabled = ?1 WHERE profile_id = ?2 AND mod_id = ?3",
        params![if enabled { 1 } else { 0 }, profile_id, mod_id],
    )?;
    if tx_sql.changes() == 0 {
        return Err(CoreError::ModNotFound);
    }
    tx::add_step(
        &tx_sql,
        &operation_id,
        1,
        "set_mod_enabled",
        &serde_json::json!({ "modId": mod_id, "enabled": enabled }).to_string(),
        Some(r#"{"compensation":"restore_previous_enabled_state"}"#),
        "done",
        step_started.elapsed().as_millis() as i64,
        None,
    )?;
    recalc_conflicts(&tx_sql, profile_id)?;
    let _deploy_state_id = persist_deploy_state(&tx_sql, profile_id, staging_root)?;
    tx::finish_transaction(&tx_sql, &operation_id, "committed")?;
    tx_sql.commit()?;

    apply_deploy_to_mod_path(conn, profile_id, staging_root)?;

    Ok(OperationResult { operation_id })
}

pub fn remove_mod(
    conn: &mut Connection,
    profile_id: &str,
    mod_id: &str,
    staging_root: &Path,
) -> Result<OperationResult, CoreError> {
    let game_id: String = conn.query_row(
        "SELECT game_id FROM mods WHERE id = ?1",
        [mod_id],
        |row| row.get(0),
    )?;
    let mod_path: String = conn.query_row(
        "SELECT mod_path FROM games WHERE id = ?1",
        [&game_id],
        |row| row.get(0),
    )?;
    let paths: Vec<String> = {
        let mut stmt = conn.prepare("SELECT relative_path FROM mod_files WHERE mod_id = ?1")?;
        let rows = stmt.query_map([mod_id], |row| row.get::<_, String>(0))?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        out
    };
    remove_deployed_files(&PathBuf::from(mod_path), &paths);

    let tx_sql = conn.transaction()?;
    let operation_id = tx::start_transaction(&tx_sql, "remove_mod", Some(profile_id))?;
    let step_started = Instant::now();

    remove_staging_dir(staging_root, mod_id)?;

    tx_sql.execute("DELETE FROM mod_files WHERE mod_id = ?1", [mod_id])?;
    tx_sql.execute("DELETE FROM profile_mods WHERE mod_id = ?1", [mod_id])?;
    tx_sql.execute("DELETE FROM mods WHERE id = ?1", [mod_id])?;
    if tx_sql.changes() == 0 {
        return Err(CoreError::ModNotFound);
    }

    tx::add_step(
        &tx_sql,
        &operation_id,
        1,
        "remove_mod",
        &serde_json::json!({ "modId": mod_id }).to_string(),
        Some(r#"{"compensation":"restore_removed_mod"}"#),
        "done",
        step_started.elapsed().as_millis() as i64,
        None,
    )?;
    recalc_conflicts(&tx_sql, profile_id)?;
    let _deploy_state_id = persist_deploy_state(&tx_sql, profile_id, staging_root)?;
    tx::finish_transaction(&tx_sql, &operation_id, "committed")?;
    tx_sql.commit()?;

    apply_deploy_to_mod_path(conn, profile_id, staging_root)?;

    Ok(OperationResult { operation_id })
}

pub fn switch_profile(
    conn: &mut Connection,
    game_id: &str,
    target_profile_id: &str,
    staging_root: &Path,
) -> Result<SwitchProfileResult, CoreError> {
    let tx_sql = conn.transaction()?;
    let operation_id = tx::start_transaction(&tx_sql, "switch_profile", Some(target_profile_id))?;
    let step_started = Instant::now();
    let profile_exists: Option<String> = tx_sql
        .query_row(
            "SELECT id FROM profiles WHERE id = ?1 AND game_id = ?2",
            params![target_profile_id, game_id],
            |row| row.get(0),
        )
        .optional()?;
    if profile_exists.is_none() {
        return Err(CoreError::ProfileNotFound);
    }
    tx_sql.execute("UPDATE profiles SET is_active = 0 WHERE game_id = ?1", [game_id])?;
    tx_sql.execute(
        "UPDATE profiles SET is_active = 1 WHERE id = ?1",
        [target_profile_id],
    )?;
    tx::add_step(
        &tx_sql,
        &operation_id,
        1,
        "switch_profile",
        &serde_json::json!({ "targetProfileId": target_profile_id }).to_string(),
        Some(r#"{"compensation":"restore_previous_active_profile"}"#),
        "done",
        step_started.elapsed().as_millis() as i64,
        None,
    )?;
    let _deploy_state_id = persist_deploy_state(&tx_sql, target_profile_id, staging_root)?;
    tx::finish_transaction(&tx_sql, &operation_id, "committed")?;
    tx_sql.commit()?;

    apply_deploy_to_mod_path(conn, target_profile_id, staging_root)?;

    Ok(SwitchProfileResult {
        operation_id,
        active_profile_id: target_profile_id.to_string(),
    })
}

pub fn reorder_mod_priority(
    conn: &mut Connection,
    profile_id: &str,
    mod_id: &str,
    move_up: bool,
    staging_root: &Path,
) -> Result<OperationResult, CoreError> {
    let tx_sql = conn.transaction()?;
    let operation_id = tx::start_transaction(&tx_sql, "reorder_mod", Some(profile_id))?;

    let mut ordered: Vec<(String, i64)> = Vec::new();
    {
        let mut stmt = tx_sql.prepare(
            "SELECT pm.mod_id, pm.priority FROM profile_mods pm
             INNER JOIN mods m ON m.id = pm.mod_id
             WHERE pm.profile_id = ?1
             ORDER BY pm.priority DESC, m.installed_at DESC",
        )?;
        let rows = stmt.query_map([profile_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;
        for r in rows {
            ordered.push(r?);
        }
    }

    let idx = ordered
        .iter()
        .position(|(id, _)| id == mod_id)
        .ok_or(CoreError::ModNotFound)?;

    if move_up && idx > 0 {
        let a_id = ordered[idx].0.clone();
        let b_id = ordered[idx - 1].0.clone();
        let pa = ordered[idx].1;
        let pb = ordered[idx - 1].1;
        tx_sql.execute(
            "UPDATE profile_mods SET priority = ?1 WHERE profile_id = ?2 AND mod_id = ?3",
            params![pb, profile_id, a_id],
        )?;
        tx_sql.execute(
            "UPDATE profile_mods SET priority = ?1 WHERE profile_id = ?2 AND mod_id = ?3",
            params![pa, profile_id, b_id],
        )?;
    } else if !move_up && idx < ordered.len().saturating_sub(1) {
        let a_id = ordered[idx].0.clone();
        let b_id = ordered[idx + 1].0.clone();
        let pa = ordered[idx].1;
        let pb = ordered[idx + 1].1;
        tx_sql.execute(
            "UPDATE profile_mods SET priority = ?1 WHERE profile_id = ?2 AND mod_id = ?3",
            params![pb, profile_id, a_id],
        )?;
        tx_sql.execute(
            "UPDATE profile_mods SET priority = ?1 WHERE profile_id = ?2 AND mod_id = ?3",
            params![pa, profile_id, b_id],
        )?;
    }

    recalc_conflicts(&tx_sql, profile_id)?;
    let _deploy_state_id = persist_deploy_state(&tx_sql, profile_id, staging_root)?;
    tx::finish_transaction(&tx_sql, &operation_id, "committed")?;
    tx_sql.commit()?;

    apply_deploy_to_mod_path(conn, profile_id, staging_root)?;

    Ok(OperationResult { operation_id })
}

pub fn recalc_conflicts(conn: &Connection, profile_id: &str) -> Result<usize, CoreError> {
    conn.execute("DELETE FROM conflicts WHERE profile_id = ?1", [profile_id])?;
    let mut stmt = conn.prepare(
        "SELECT m.id, pm.priority, f.relative_path
         FROM mods m
         INNER JOIN profile_mods pm ON pm.mod_id = m.id AND pm.profile_id = ?1
         INNER JOIN mod_files f ON f.mod_id = m.id
         WHERE pm.enabled = 1
         ORDER BY pm.priority DESC, m.installed_at DESC",
    )?;
    let rows = stmt.query_map([profile_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    let mut target_map: HashMap<String, Vec<(String, i64)>> = HashMap::new();
    for row in rows {
        let (mod_id, priority, relative_path) = row?;
        let key = relative_path.replace('\\', "/");
        target_map.entry(key).or_default().push((mod_id, priority));
    }

    let mut inserted = 0usize;
    for (target_path, mut mods) in target_map {
        if mods.len() < 2 {
            continue;
        }
        mods.sort_by(|a, b| b.1.cmp(&a.1));
        let winner = mods[0].0.clone();
        let losers: Vec<String> = mods.into_iter().skip(1).map(|item| item.0).collect();
        conn.execute(
            "INSERT INTO conflicts (id, profile_id, target_path, winner_mod_id, loser_mod_ids_json, resolved_by, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 'priority', ?6)",
            params![
                Uuid::new_v4().to_string(),
                profile_id,
                target_path,
                winner,
                serde_json::to_string(&losers).unwrap_or_else(|_| "[]".to_string()),
                now()
            ],
        )?;
        inserted += 1;
    }
    Ok(inserted)
}

fn persist_deploy_state(conn: &Connection, profile_id: &str, staging_root: &Path) -> Result<String, CoreError> {
    conn.execute(
        "UPDATE deploy_state SET is_current = 0 WHERE profile_id = ?1",
        [profile_id],
    )?;
    let deploy_id = Uuid::new_v4().to_string();
    let mut entries_stmt = conn.prepare(
        "SELECT f.relative_path, f.checksum, m.id
         FROM mod_files f
         INNER JOIN mods m ON m.id = f.mod_id
         INNER JOIN profile_mods pm ON pm.mod_id = m.id AND pm.profile_id = ?1
         WHERE pm.enabled = 1
         ORDER BY pm.priority DESC, m.installed_at DESC",
    )?;
    let entry_rows = entries_stmt.query_map([profile_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, Option<String>>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    let mut entries = Vec::new();
    for row in entry_rows {
        let (target_path, checksum, winner_mod_id) = row?;
        let staging_rel = staging_root
            .join(&winner_mod_id)
            .join(target_path.trim_start_matches('/'));
        let staging_display = staging_rel.to_string_lossy().to_string();
        entries.push(serde_json::json!({
            "targetPath": target_path,
            "winnerModId": winner_mod_id,
            "sourcePath": staging_display,
            "strategy": "copy",
            "checksum": checksum.unwrap_or_else(|| "sha256:unknown".to_string())
        }));
    }

    let manifest_json = serde_json::json!({
      "manifestId": deploy_id,
      "profileId": profile_id,
      "createdAt": now(),
      "entries": entries
    })
    .to_string();
    conn.execute(
        "INSERT INTO deploy_state (id, profile_id, manifest_json, created_at, is_current) VALUES (?1, ?2, ?3, ?4, 1)",
        params![deploy_id, profile_id, manifest_json, now()],
    )?;
    Ok(deploy_id)
}
