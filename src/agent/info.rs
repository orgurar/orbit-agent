use crate::agent;

use serde::Serialize;

// a struct that will represent the agent's initial info about the hosting system
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    hostname: String,
    username: String,
    win_version: String,
    curr_dir: String,
    is_vm: bool,
}

impl Info {
    pub fn collect() -> Self {
        let hostname = agent::utils::hostname();
        let username = agent::utils::username();
        let win_version = agent::utils::win_version();
        let curr_dir = agent::utils::curr_path();
        let is_vm = agent::utils::on_vm();

        Self {
            hostname,
            username,
            win_version,
            curr_dir,
            is_vm,
        }
    }
}
