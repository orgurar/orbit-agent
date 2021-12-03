use crate::agent;

// a struct that will represent the agent's initial info about the hosting system
#[derive(Debug)]
pub struct Info {
    hostname: String,
    username: String,
    win_version: String,
    is_vm: bool,
}

impl Info {
    pub fn collect() -> Info {
        let hostname = agent::utils::hostname();
        let username = agent::utils::username();
        let win_version = agent::utils::win_version();
        let is_vm = agent::utils::on_vm();

        Info {
            hostname,
            username,
            win_version,
            is_vm,
        }
    }

    pub fn as_string(&self) -> String {
        format!(
            "{{ \"hostname\": \"{}\", \"username\": \"{}\", \"winVersion\": \"{}\", \"isVM\": {} }}",
            self.hostname, self.username, self.win_version, self.is_vm
        )
    }
}
