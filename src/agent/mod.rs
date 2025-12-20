use std::{
    sync::mpsc::{Receiver, Sender},
    time::Instant,
};

use crate::{agent, app};

pub mod input;
pub mod sessions;
pub mod tasks;

pub enum AgentCommand {
    StartSession {
        id: i64,
    },
    EndSession {
        comment: String,
    },
    UserActive {
        time_stamp: chrono::DateTime<chrono::Utc>,
    },
    AddTask {
        task: agent::tasks::Task,
    },
    RequestTaskList,
    RequestTaskState,
}

pub fn start_agent(
    agent_rx: Receiver<AgentCommand>,
    app_tx: Sender<app::AppCommand>,
    mut app_state: app::types::AppState,
) {
    std::thread::spawn(move || {
        loop {
            while let Ok(event) = agent_rx.try_recv() {
                match event {
                    AgentCommand::StartSession { id } => {
                        app_state.task_in_progress = true;
                        app_state.session = agent::sessions::Session::default();
                        app_state.session.s_task = id;
                    }
                    AgentCommand::EndSession { comment } => {
                        app_state.task_in_progress = false;
                        let end_time = Instant::now();
                        app_state.session.s_duration = (end_time - app_state.start_time).as_secs();
                        app_state.session.s_comment = comment;
                        agent::sessions::save_session(&app_state.db_connection, &app_state.session)
                            .unwrap();
                    }
                    AgentCommand::AddTask { task } => {
                        agent::tasks::add_new_task(&app_state.db_connection, &task).unwrap();
                    }
                    AgentCommand::RequestTaskList => {
                        let task_list =
                            agent::tasks::get_all_tasks(&app_state.db_connection).unwrap();
                        app_tx
                            .send(app::AppCommand::TaskList { task_list })
                            .unwrap();
                    }
                    AgentCommand::RequestTaskState => {
                        let state = app_state.task_in_progress;
                        app_tx
                            .send(app::AppCommand::ProgressState { state })
                            .unwrap();
                    }
                    _ => (),
                }
            }
        }
    });
}
