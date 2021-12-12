pub const CLI: &str = "cmd.exe"; // Command Line Interface
pub const UNKNOWN_COMMAND: &str = "Orbit Error: Unknown Command"; // unknown command error string
pub const NOT_IMPLEMENTED: &str = "Orbit Error: Not implemented yet"; // not implemented error message
pub const OUTPUT_ERR: &str = "Cannot Read Command's Output"; // invalid output error string
pub const HELP: &str = "Orbit Agent Commands:
[+]\t'cmd' - Run a CMD command
[+]\t'ps' - Run a PowerShell command
[+]\t'download_exec' - Download file from a given URL and execute it
[+]\t'delete_agent' - Agent's self deletion
[+]\t'exit_agent' - Stops Agent from running
[?]\t'echo' - check if a certain agent is alive
[?]\t'version' - Print Agent's version string
[?]\t'help' - Print help instructions"; // help string
