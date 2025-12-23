use std::sync::mpsc;

use crate::{
    agent,
    ui::{self},
};

pub fn start() {
    std::fs::create_dir_all("data").unwrap();

    let (command_tx, command_rx) = mpsc::channel();
    let (event_tx, event_rx) = mpsc::channel();

    // Start local agent
    agent::input::start_input_listener(command_tx.clone());
    agent::start_agent(command_rx, event_tx);

    // Initialize tray icon
    let tray = ui::tray::Tray::init_tray_icon(command_tx.clone());
    tray.start_tray_icon();

    // Open ui
    ui::window::run_ui(command_tx, event_rx);
}
