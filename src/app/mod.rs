use crate::agent;

pub mod startup;
pub mod types;

pub enum AppCommand {
    TaskList { task_list: Vec<agent::tasks::Task> },
    ProgressState { state: bool },
}
