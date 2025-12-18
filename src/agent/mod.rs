pub mod input;
pub mod tasks;

pub enum AgentCommand {
    StartTask {
        name: String,
    },
    StopTask,
    UserActive {
        time_stamp: chrono::DateTime<chrono::Utc>,
    },
}
