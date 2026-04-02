use std::path::PathBuf;

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Transaction};

use crate::core::errors::CoreError;

struct Migration {
    version: i64,
    name: &'static str,
    sql: &'static str,
}

const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "001_initial",
        sql: r#"
        CREATE TABLE IF NOT EXISTS games (
          id TEXT PRIMARY KEY,
          name TEXT NOT NULL,
          install_path TEXT NOT NULL,
          mod_path TEXT NOT NULL,
          created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS profiles (
          id TEXT PRIMARY KEY,
          game_id TEXT NOT NULL REFERENCES games(id),
          name TEXT NOT NULL,
          is_active INTEGER NOT NULL DEFAULT 0,
          created_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_profiles_game_id ON profiles(game_id);
        CREATE INDEX IF NOT EXISTS idx_profiles_active_game ON profiles(game_id, is_active);
        CREATE UNIQUE INDEX IF NOT EXISTS uq_profiles_one_active_per_game ON profiles(game_id) WHERE is_active = 1;

        CREATE TABLE IF NOT EXISTS mods (
          id TEXT PRIMARY KEY,
          game_id TEXT NOT NULL REFERENCES games(id),
          name TEXT NOT NULL,
          version TEXT,
          archive_path TEXT NOT NULL,
          enabled INTEGER NOT NULL DEFAULT 1,
          priority INTEGER NOT NULL DEFAULT 0,
          installed_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_mods_game_id ON mods(game_id);
        CREATE INDEX IF NOT EXISTS idx_mods_game_priority ON mods(game_id, priority DESC);

        CREATE TABLE IF NOT EXISTS mod_files (
          id TEXT PRIMARY KEY,
          mod_id TEXT NOT NULL REFERENCES mods(id),
          relative_path TEXT NOT NULL,
          checksum TEXT,
          size_bytes INTEGER
        );
        CREATE INDEX IF NOT EXISTS idx_mod_files_mod_id ON mod_files(mod_id);
        CREATE INDEX IF NOT EXISTS idx_mod_files_rel_path ON mod_files(relative_path);

        CREATE TABLE IF NOT EXISTS deploy_state (
          id TEXT PRIMARY KEY,
          profile_id TEXT NOT NULL REFERENCES profiles(id),
          manifest_json TEXT NOT NULL,
          created_at TEXT NOT NULL,
          is_current INTEGER NOT NULL DEFAULT 1
        );
        CREATE INDEX IF NOT EXISTS idx_deploy_state_profile_current ON deploy_state(profile_id, is_current);
        CREATE UNIQUE INDEX IF NOT EXISTS uq_deploy_state_one_current_per_profile ON deploy_state(profile_id) WHERE is_current = 1;

        CREATE TABLE IF NOT EXISTS conflicts (
          id TEXT PRIMARY KEY,
          profile_id TEXT NOT NULL REFERENCES profiles(id),
          target_path TEXT NOT NULL,
          winner_mod_id TEXT NOT NULL REFERENCES mods(id),
          loser_mod_ids_json TEXT NOT NULL,
          resolved_by TEXT NOT NULL,
          updated_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_conflicts_profile ON conflicts(profile_id);
        CREATE INDEX IF NOT EXISTS idx_conflicts_target_path ON conflicts(profile_id, target_path);
        "#,
    },
    Migration {
        version: 2,
        name: "002_transactions",
        sql: r#"
        CREATE TABLE IF NOT EXISTS transactions (
          id TEXT PRIMARY KEY,
          operation_type TEXT NOT NULL,
          profile_id TEXT,
          status TEXT NOT NULL,
          started_at TEXT NOT NULL,
          finished_at TEXT,
          error_code TEXT,
          error_message TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_tx_status ON transactions(status);
        CREATE INDEX IF NOT EXISTS idx_tx_profile_started ON transactions(profile_id, started_at DESC);

        CREATE TABLE IF NOT EXISTS transaction_steps (
          id TEXT PRIMARY KEY,
          transaction_id TEXT NOT NULL REFERENCES transactions(id),
          step_order INTEGER NOT NULL,
          step_type TEXT NOT NULL,
          payload_json TEXT NOT NULL,
          compensation_json TEXT,
          status TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_tx_steps_txid ON transaction_steps(transaction_id);
        CREATE INDEX IF NOT EXISTS idx_tx_steps_order ON transaction_steps(transaction_id, step_order);
        "#,
    },
    Migration {
        version: 3,
        name: "003_transaction_steps",
        sql: r#"
        ALTER TABLE transaction_steps ADD COLUMN duration_ms INTEGER NOT NULL DEFAULT 0;
        ALTER TABLE transaction_steps ADD COLUMN error_code TEXT;
        "#,
    },
    Migration {
        version: 4,
        name: "004_profile_mods",
        sql: r#"
        CREATE TABLE IF NOT EXISTS profile_mods (
          profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
          mod_id TEXT NOT NULL REFERENCES mods(id) ON DELETE CASCADE,
          enabled INTEGER NOT NULL DEFAULT 1,
          priority INTEGER NOT NULL DEFAULT 0,
          PRIMARY KEY (profile_id, mod_id)
        );
        CREATE INDEX IF NOT EXISTS idx_profile_mods_profile ON profile_mods(profile_id);
        CREATE INDEX IF NOT EXISTS idx_profile_mods_profile_pri ON profile_mods(profile_id, priority DESC);

        INSERT OR IGNORE INTO profile_mods (profile_id, mod_id, enabled, priority)
        SELECT p.id, m.id, m.enabled, m.priority
        FROM mods m
        INNER JOIN profiles p ON p.game_id = m.game_id;

        -- DROP COLUMN cannot run while an index still references the column (001_initial).
        DROP INDEX IF EXISTS idx_mods_game_priority;

        ALTER TABLE mods DROP COLUMN enabled;
        ALTER TABLE mods DROP COLUMN priority;
        "#,
    },
];

pub struct CoreDb {
    pub conn: Connection,
}

impl CoreDb {
    pub fn new(db_path: PathBuf) -> Result<Self, CoreError> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| CoreError::InvalidInput)?;
        }
        let conn = Connection::open(db_path)?;
        conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            ",
        )?;
        run_migrations(&conn)?;
        seed_if_empty(&conn)?;
        Ok(Self { conn })
    }
}

fn run_migrations(conn: &Connection) -> Result<(), CoreError> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS schema_migrations (
          version INTEGER PRIMARY KEY,
          name TEXT NOT NULL,
          applied_at TEXT NOT NULL
        );
        ",
    )?;

    for migration in MIGRATIONS {
        let exists: Option<i64> = conn
            .query_row(
                "SELECT version FROM schema_migrations WHERE version = ?1",
                [migration.version],
                |row| row.get(0),
            )
            .optional()?;
        if exists.is_some() {
            continue;
        }
        let tx = conn.unchecked_transaction()?;
        apply_migration(&tx, migration)?;
        tx.commit()?;
    }
    Ok(())
}

fn apply_migration(tx: &Transaction<'_>, migration: &Migration) -> Result<(), CoreError> {
    tx.execute_batch(migration.sql)?;
    tx.execute(
        "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
        params![migration.version, migration.name, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

fn seed_if_empty(conn: &Connection) -> Result<(), CoreError> {
    let game_exists: Option<String> = conn
        .query_row("SELECT id FROM games LIMIT 1", [], |row| row.get(0))
        .optional()?;
    if game_exists.is_none() {
        let created_at = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO games (id, name, install_path, mod_path, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params!["pilot-game", "Pilot Game", "C:/Games/PilotGame", "C:/Games/PilotGame/Mods", created_at],
        )?;
        conn.execute(
            "INSERT INTO profiles (id, game_id, name, is_active, created_at) VALUES (?1, ?2, ?3, 1, ?4)",
            params!["default-profile", "pilot-game", "Default", Utc::now().to_rfc3339()],
        )?;
        conn.execute(
            "INSERT INTO profiles (id, game_id, name, is_active, created_at) VALUES (?1, ?2, ?3, 0, ?4)",
            params!["secondary-profile", "pilot-game", "Secondary", Utc::now().to_rfc3339()],
        )?;
    }
    Ok(())
}
