// command kind with agent's builtin options
pub enum CommandKind {
    Cmd,
    Delete,
    Exit,
    Help,
    None,
    Unknown,
}

impl CommandKind {
    pub fn parse_kind(s: &String) -> Self {
        match s.as_str() {
            "cmd" => Self::Cmd,
            "delete_agent" => Self::Delete,
            "exit_agent" => Self::Exit,
            "help" => Self::Help,
            "" => Self::None,
            _ => Self::Unknown,
        }
    }
}
