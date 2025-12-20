use std::sync::mpsc::Sender;

use rdev::{Event, listen};

use crate::agent;

pub fn start_input_listener(tx: Sender<agent::AgentCommand>) {
    std::thread::spawn(move || {
        let _ = listen(move |_event: Event| {
            let _ = tx.send(agent::AgentCommand::UserActive {
                time_stamp: chrono::Utc::now(),
            });
        });
    });
}
