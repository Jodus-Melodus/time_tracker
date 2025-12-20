use std::time::Instant;

use rusqlite::Connection;
use tray_icon;

use crate::{agent, ui};

pub struct AppState {
    pub _tray_icon: tray_icon::TrayIcon,
    pub db_connection: Connection,

    pub session: agent::sessions::Session,
    pub start_time: Instant,
    pub task_in_progress: bool,
}

impl AppState {
    pub fn new(db_connection: Connection) -> Self {
        AppState {
            _tray_icon: ui::tray::init_tray_icon(),
            db_connection,

            session: agent::sessions::Session::default(),
            start_time: Instant::now(),
            task_in_progress: false,
        }
    }
}

#[derive(PartialEq)]
pub enum UserState {
    Idle,
    Active,
}
