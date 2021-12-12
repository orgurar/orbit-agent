// command kind with agent's builtin options
pub enum CommandKind {
    Cmd,
    PowerShell,
    DownloadExec,
    Delete,
    Exit,
    Echo,
    Version,
    Help,
    None,
    Unknown,
}

impl CommandKind {
    pub fn parse_kind(s: &String) -> Self {
        match s.as_str() {
            "cmd" => Self::Cmd,
            "ps" => Self::PowerShell,
            "download_exec" => Self::DownloadExec,
            "delete_agent" => Self::Delete,
            "exit_agent" => Self::Exit,
            "echo" => Self::Echo,
            "version" => Self::Version,
            "help" => Self::Help,
            "" => Self::None,
            _ => Self::Unknown,
        }
    }
}
