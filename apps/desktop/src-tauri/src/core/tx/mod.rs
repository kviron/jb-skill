use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::core::errors::CoreError;

pub fn start_transaction(
    conn: &Connection,
    operation_type: &str,
    profile_id: Option<&str>,
) -> Result<String, CoreError> {
    let tx_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO transactions (id, operation_type, profile_id, status, started_at) VALUES (?1, ?2, ?3, 'started', ?4)",
        params![tx_id, operation_type, profile_id, Utc::now().to_rfc3339()],
    )?;
    Ok(tx_id)
}

pub fn add_step(
    conn: &Connection,
    transaction_id: &str,
    step_order: i64,
    step_type: &str,
    payload_json: &str,
    compensation_json: Option<&str>,
    status: &str,
    duration_ms: i64,
    error_code: Option<&str>,
) -> Result<String, CoreError> {
    let step_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO transaction_steps (id, transaction_id, step_order, step_type, payload_json, compensation_json, status, duration_ms, error_code)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            step_id,
            transaction_id,
            step_order,
            step_type,
            payload_json,
            compensation_json,
            status,
            duration_ms,
            error_code
        ],
    )?;
    Ok(step_id)
}

pub fn mark_step_status(
    conn: &Connection,
    step_id: &str,
    status: &str,
    duration_ms: Option<i64>,
    error_code: Option<&str>,
) -> Result<(), CoreError> {
    conn.execute(
        "UPDATE transaction_steps SET status = ?1, duration_ms = COALESCE(?2, duration_ms), error_code = ?3 WHERE id = ?4",
        params![status, duration_ms, error_code, step_id],
    )?;
    Ok(())
}

pub fn finish_transaction(conn: &Connection, transaction_id: &str, status: &str) -> Result<(), CoreError> {
    conn.execute(
        "UPDATE transactions SET status = ?1, finished_at = ?2 WHERE id = ?3",
        params![status, Utc::now().to_rfc3339(), transaction_id],
    )?;
    Ok(())
}

pub fn recover_started_transactions(conn: &Connection) -> Result<Vec<String>, CoreError> {
    let mut stmt = conn.prepare("SELECT id FROM transactions WHERE status = 'started'")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    let mut recovered = Vec::new();
    for row in rows {
        let tx_id = row?;
        let mut steps_stmt = conn.prepare(
            "SELECT id, compensation_json FROM transaction_steps
             WHERE transaction_id = ?1 AND status = 'done'
             ORDER BY step_order DESC",
        )?;
        let steps = steps_stmt.query_map([&tx_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?))
        })?;
        let mut rollback_failed = false;
        for step in steps {
            let (step_id, compensation_json) = step?;
            if compensation_json.is_none() {
                continue;
            }
            if let Err(_err) = mark_step_status(conn, &step_id, "compensated", None, None) {
                rollback_failed = true;
                let _ = mark_step_status(conn, &step_id, "failed", None, Some("ROLLBACK_FAILED"));
            }
        }
        let (status, error_code, error_message) = if rollback_failed {
            ("failed", "ROLLBACK_FAILED", "Rollback failed during recovery")
        } else {
            ("rolled_back", "RECOVERY_RUN", "Recovered on startup")
        };
        conn.execute(
            "UPDATE transactions SET status = ?1, finished_at = ?2, error_code = ?3, error_message = ?4 WHERE id = ?5",
            params![status, Utc::now().to_rfc3339(), error_code, error_message, tx_id],
        )?;
        recovered.push(tx_id);
    }
    Ok(recovered)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute_batch(
            "
            CREATE TABLE transactions (
              id TEXT PRIMARY KEY,
              operation_type TEXT NOT NULL,
              profile_id TEXT,
              status TEXT NOT NULL,
              started_at TEXT NOT NULL,
              finished_at TEXT,
              error_code TEXT,
              error_message TEXT
            );
            CREATE TABLE transaction_steps (
              id TEXT PRIMARY KEY,
              transaction_id TEXT NOT NULL,
              step_order INTEGER NOT NULL,
              step_type TEXT NOT NULL,
              payload_json TEXT NOT NULL,
              compensation_json TEXT,
              status TEXT NOT NULL,
              duration_ms INTEGER NOT NULL DEFAULT 0,
              error_code TEXT
            );
            ",
        )
        .expect("schema");
        conn
    }

    #[test]
    fn recovers_started_transaction() {
        let conn = setup_conn();
        let tx_id = start_transaction(&conn, "install", Some("default-profile")).expect("start");
        let _step_id = add_step(
            &conn,
            &tx_id,
            1,
            "stage_files",
            "{}",
            Some("{\"compensation\":\"delete_staging_dir\"}"),
            "done",
            10,
            None,
        )
        .expect("step");
        let recovered = recover_started_transactions(&conn).expect("recover");
        assert_eq!(recovered.len(), 1);
        let status: String = conn
            .query_row("SELECT status FROM transactions WHERE id = ?1", [&tx_id], |row| row.get(0))
            .expect("tx status");
        assert_eq!(status, "rolled_back");
    }
}
