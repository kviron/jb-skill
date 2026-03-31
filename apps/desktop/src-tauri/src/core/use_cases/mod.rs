use std::{collections::HashMap, path::PathBuf, time::Instant};

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

use crate::core::{
    contracts::{ConflictRecord, InstallResult, ModRecord, OperationResult, SwitchProfileResult},
    errors::CoreError,
    plugins,
    tx,
};

fn now() -> String {
    Utc::now().to_rfc3339()
}

pub fn list_mods(conn: &Connection, game_id: &str) -> Result<Vec<ModRecord>, CoreError> {
    let mut stmt = conn.prepare(
        "SELECT id, game_id, name, version, archive_path, enabled, priority, installed_at
         FROM mods WHERE game_id = ?1 ORDER BY priority DESC, installed_at DESC",
    )?;

    let rows = stmt.query_map([game_id], |row| {
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

pub fn install_mod_from_archive(
    conn: &mut Connection,
    game_id: &str,
    profile_id: &str,
    archive_path: &str,
) -> Result<InstallResult, CoreError> {
    if archive_path.trim().is_empty() {
        return Err(CoreError::InvalidInput);
    }
    let tx_sql = conn.transaction()?;
    let operation_id = tx::start_transaction(&tx_sql, "install", Some(profile_id))?;
    let manifest = plugins::load_manifest(&plugins::plugin_root().join("game-pilot").join("manifest.json"))?;

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

    let mod_id = Uuid::new_v4().to_string();
    let parsed_name = PathBuf::from(archive_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("New Mod")
        .to_string();

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

    let max_priority: i64 = tx_sql.query_row(
        "SELECT COALESCE(MAX(priority), 0) FROM mods WHERE game_id = ?1",
        [game_id],
        |row| row.get(0),
    )?;
    tx_sql.execute(
        "INSERT INTO mods (id, game_id, name, version, archive_path, enabled, priority, installed_at)
         VALUES (?1, ?2, ?3, '1.0.0', ?4, 1, ?5, ?6)",
        params![mod_id, game_id, parsed_name, archive_path, max_priority + 1, now()],
    )?;
    tx_sql.execute(
        "INSERT INTO mod_files (id, mod_id, relative_path, checksum, size_bytes) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            Uuid::new_v4().to_string(),
            mod_id,
            format!("Data/{}.esp", PathBuf::from(archive_path).file_stem().and_then(|s| s.to_str()).unwrap_or("mod").to_lowercase()),
            format!("sha256:{}", Uuid::new_v4()),
            0
        ],
    )?;
    let install_plan_started = Instant::now();
    let install_plan = plugins::plan_install(&manifest, archive_path, &mod_id)?;
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
    let deploy_plan = plugins::plan_deploy(&manifest, profile_id, &mod_id)?;
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
    let _deploy_state_id = persist_deploy_state(&tx_sql, profile_id)?;
    recalc_conflicts(&tx_sql, profile_id)?;
    tx::finish_transaction(&tx_sql, &operation_id, "committed")?;
    tx_sql.commit()?;

    Ok(InstallResult {
        operation_id,
        mod_id,
        warnings: vec![],
    })
}

pub fn set_mod_enabled(
    conn: &mut Connection,
    profile_id: &str,
    mod_id: &str,
    enabled: bool,
) -> Result<OperationResult, CoreError> {
    let tx_sql = conn.transaction()?;
    let operation_id = tx::start_transaction(&tx_sql, "set_enabled", Some(profile_id))?;
    let step_started = Instant::now();
    tx_sql.execute(
        "UPDATE mods SET enabled = ?1 WHERE id = ?2",
        params![if enabled { 1 } else { 0 }, mod_id],
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
    let _deploy_state_id = persist_deploy_state(&tx_sql, profile_id)?;
    tx::finish_transaction(&tx_sql, &operation_id, "committed")?;
    tx_sql.commit()?;
    Ok(OperationResult { operation_id })
}

pub fn remove_mod(conn: &mut Connection, profile_id: &str, mod_id: &str) -> Result<OperationResult, CoreError> {
    let tx_sql = conn.transaction()?;
    let operation_id = tx::start_transaction(&tx_sql, "remove_mod", Some(profile_id))?;
    let step_started = Instant::now();
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
    let _deploy_state_id = persist_deploy_state(&tx_sql, profile_id)?;
    tx::finish_transaction(&tx_sql, &operation_id, "committed")?;
    tx_sql.commit()?;
    Ok(OperationResult { operation_id })
}

pub fn switch_profile(
    conn: &mut Connection,
    game_id: &str,
    target_profile_id: &str,
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
    let _deploy_state_id = persist_deploy_state(&tx_sql, target_profile_id)?;
    tx::finish_transaction(&tx_sql, &operation_id, "committed")?;
    tx_sql.commit()?;
    Ok(SwitchProfileResult {
        operation_id,
        active_profile_id: target_profile_id.to_string(),
    })
}

pub fn recalc_conflicts(conn: &Connection, profile_id: &str) -> Result<usize, CoreError> {
    conn.execute("DELETE FROM conflicts WHERE profile_id = ?1", [profile_id])?;
    let mut stmt = conn.prepare(
        "SELECT m.id, m.priority, f.relative_path
         FROM mods m
         JOIN mod_files f ON f.mod_id = m.id
         WHERE m.enabled = 1
         ORDER BY m.priority DESC, m.installed_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
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

fn persist_deploy_state(conn: &Connection, profile_id: &str) -> Result<String, CoreError> {
    conn.execute(
        "UPDATE deploy_state SET is_current = 0 WHERE profile_id = ?1",
        [profile_id],
    )?;
    let deploy_id = Uuid::new_v4().to_string();
    let mut entries_stmt = conn.prepare(
        "SELECT f.relative_path, f.checksum, m.id
         FROM mod_files f
         JOIN mods m ON m.id = f.mod_id
         WHERE m.enabled = 1
         ORDER BY m.priority DESC, m.installed_at DESC",
    )?;
    let entry_rows = entries_stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, Option<String>>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    let mut entries = Vec::new();
    for row in entry_rows {
        let (target_path, checksum, winner_mod_id) = row?;
        entries.push(serde_json::json!({
            "targetPath": target_path,
            "winnerModId": winner_mod_id,
            "sourcePath": format!("mods/{winner_mod_id}/{}", target_path),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute_batch(
            "
            CREATE TABLE profiles (id TEXT PRIMARY KEY, game_id TEXT NOT NULL, name TEXT NOT NULL, is_active INTEGER NOT NULL, created_at TEXT NOT NULL);
            CREATE TABLE mods (id TEXT PRIMARY KEY, game_id TEXT NOT NULL, name TEXT NOT NULL, version TEXT, archive_path TEXT NOT NULL, enabled INTEGER NOT NULL, priority INTEGER NOT NULL, installed_at TEXT NOT NULL);
            CREATE TABLE mod_files (id TEXT PRIMARY KEY, mod_id TEXT NOT NULL, relative_path TEXT NOT NULL, checksum TEXT, size_bytes INTEGER);
            CREATE TABLE conflicts (id TEXT PRIMARY KEY, profile_id TEXT NOT NULL, target_path TEXT NOT NULL, winner_mod_id TEXT NOT NULL, loser_mod_ids_json TEXT NOT NULL, resolved_by TEXT NOT NULL, updated_at TEXT NOT NULL);
            CREATE TABLE deploy_state (id TEXT PRIMARY KEY, profile_id TEXT NOT NULL, manifest_json TEXT NOT NULL, created_at TEXT NOT NULL, is_current INTEGER NOT NULL);
            CREATE TABLE transactions (id TEXT PRIMARY KEY, operation_type TEXT NOT NULL, profile_id TEXT, status TEXT NOT NULL, started_at TEXT NOT NULL, finished_at TEXT, error_code TEXT, error_message TEXT);
            CREATE TABLE transaction_steps (id TEXT PRIMARY KEY, transaction_id TEXT NOT NULL, step_order INTEGER NOT NULL, step_type TEXT NOT NULL, payload_json TEXT NOT NULL, compensation_json TEXT, status TEXT NOT NULL, duration_ms INTEGER NOT NULL DEFAULT 0, error_code TEXT);
            INSERT INTO profiles (id, game_id, name, is_active, created_at) VALUES ('default-profile', 'pilot-game', 'Default', 1, '2026-01-01T00:00:00Z');
            INSERT INTO profiles (id, game_id, name, is_active, created_at) VALUES ('secondary-profile', 'pilot-game', 'Secondary', 0, '2026-01-01T00:00:00Z');
            ",
        )
        .expect("schema");
        conn
    }

    #[test]
    fn install_and_switch_profile_flow() {
        let mut conn = setup_conn();
        let install = install_mod_from_archive(&mut conn, "pilot-game", "default-profile", "C:/mods/a.zip")
            .expect("install");
        assert!(!install.mod_id.is_empty());

        let switched = switch_profile(&mut conn, "pilot-game", "secondary-profile").expect("switch");
        assert_eq!(switched.active_profile_id, "secondary-profile");
    }
}
