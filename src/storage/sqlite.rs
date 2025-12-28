use rusqlite::{Connection, Result};
use std::{fs, sync::Arc};

use crate::config;

pub fn init_db(settings: Arc<config::settings::Settings>) -> Result<Connection> {
    let conn = Connection::open(&settings.local_database_path)?;

    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        ",
    )?;

    let schema = fs::read_to_string(&settings.schema_path).unwrap();
    conn.execute_batch(&schema)?;
    // Ensure the current user exists so sessions can reference it (foreign key)
    conn.execute(
        "INSERT OR IGNORE INTO users (u_id, u_name) VALUES (?1, ?2)",
        (&settings.uid, &settings.uid),
    )?;
    Ok(conn)
}
