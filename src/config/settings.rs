use std::fs;

use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = "assets/settings.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub auto_sync_interval_seconds: u64,
    pub active_timeout_seconds: u64,
    pub icon_path: String,
    pub schema_path: String,
    pub local_database_path: String,
}

impl Settings {
    pub fn load() -> Self {
        let settings = if let Ok(data) = fs::read_to_string(SETTINGS_PATH) {
            if let Ok(settings) = serde_json::from_str::<Self>(&data) {
                settings
            } else {
                Self::default()
            }
        } else {
            Self::default()
        };

        let _ = Self::save(&settings);
        settings
    }

    pub fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(SETTINGS_PATH, json)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_sync_interval_seconds: 30,
            active_timeout_seconds: 15,
            icon_path: "assets/icon.ico".into(),
            schema_path: "assets/schema.sql".into(),
            local_database_path: "assets/sessions.db".into(),
        }
    }
}
