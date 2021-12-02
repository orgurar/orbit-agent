extern crate winreg;
use crate::config;

use std::io;
use std::path::Path;

use winreg::enums::*;
use winreg::RegKey;

// add the agent to startup register (HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Run)
pub fn add_to_startup() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let (key, _disp) = hkcu.create_subkey(&path).unwrap();

    // sets a key-value pair with name and path
    key.set_value(config::NAME, &config::filesystem::AGENT_PATH)
        .unwrap();
}

// remove agent from startup register
pub fn rm_from_startup() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let subkey = hkcu.open_subkey(&path).unwrap();

    // check if exists
    let key_handler: io::Result<String> = subkey.get_value(config::NAME);
    if let Ok(_) = key_handler {
        let (key, _disp) = hkcu.create_subkey(&path).unwrap();
        key.delete_value(config::NAME).unwrap();
    }
}
