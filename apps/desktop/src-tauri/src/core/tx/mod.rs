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
) -> Result<String, CoreError> {
    let step_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO transaction_steps (id, transaction_id, step_order, step_type, payload_json, compensation_json, status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'done')",
        params![step_id, transaction_id, step_order, step_type, payload_json, compensation_json],
    )?;
    Ok(step_id)
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
        conn.execute(
            "UPDATE transactions SET status = 'rolled_back', finished_at = ?1, error_code = 'RECOVERY_RUN', error_message = 'Recovered on startup' WHERE id = ?2",
            params![Utc::now().to_rfc3339(), tx_id],
        )?;
        recovered.push(tx_id);
    }
    Ok(recovered)
}
