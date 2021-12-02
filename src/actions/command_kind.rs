// command kind with agent's builtin options
pub enum CommandKind {
    Cmd,
    Delete,
    Exit,
    None,
    Unknown,
}

impl CommandKind {
    pub fn parse_kind(s: &String) -> CommandKind {
        match s.as_str() {
            "cmd" => CommandKind::Cmd,
            "delete_agent" => CommandKind::Delete,
            "exit_agent" => CommandKind::Exit,
            "" => CommandKind::None,
            _ => CommandKind::Unknown,
        }
    }
}
