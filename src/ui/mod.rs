use std::time::Duration;

use chrono::{DateTime, Utc};
use egui::IconData;

use crate::agent;

pub mod tray;
pub mod window;

pub enum UIEvent {
    TaskList { task_list: Vec<agent::tasks::Task> },
    UserActivity { time_stamp: DateTime<Utc> },
    ElapsedTime { elapsed: Duration },
    Quit,
}

pub enum UIControl {
    Show,
    Quit,
}

fn load_icon_from_bytes(bytes: &[u8]) -> IconData {
    let image = image::load_from_memory(bytes)
        .expect("Failed to load icon bytes")
        .into_rgba8();
    let (w, h) = image.dimensions();

    IconData {
        rgba: image.into_raw(),
        width: w,
        height: h,
    }
}
