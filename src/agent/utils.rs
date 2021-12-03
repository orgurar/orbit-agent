// utils for agent's info gathering
pub fn hostname() -> String {
    let hostname = gethostname::gethostname()
        .into_string()
        .unwrap_or(String::from("Invalid Hostname"));

    hostname
}

pub fn username() -> String {
    let username = whoami::username();

    username
}

pub fn win_version() -> String {
    let win_version = os_info::get();
    let win_version: String = format!("{}", win_version);

    win_version
}

pub fn on_vm() -> bool {
    false // implement this function
}
