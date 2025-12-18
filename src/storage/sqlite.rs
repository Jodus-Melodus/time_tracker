use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("data/sessions.db")?;
    conn.execute_batch(
        "
        CREATE TABLE tasks (
            id	INTEGER NOT NULL UNIQUE,
            name	TEXT NOT NULL DEFAULT ' ',
            description	TEXT NOT NULL DEFAULT ' ',
            priority	INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY(id AUTOINCREMENT)
        );

        CREATE TABLE sessions (
            id TEXT PRIMARY KEY,
            task_id TEXT,
            start_utc TEXT NOT NULL,
            end_utc TEXT,
            duration_sec INTEGER,
            synced INTEGER DEFAULT 0
        );

        PRAGMA journal_mode=WAL;
    ",
    )?;
    Ok(conn)
}
