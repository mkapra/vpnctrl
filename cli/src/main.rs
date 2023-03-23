use std::process;

use color_print::cprintln;
use inquire::{required, Text};

mod commands;
use commands::COMMANDS;
mod queries;

/// Holds all the values that are necessary between different commands
#[derive(Debug)]
pub struct State {
    pub url: String,
    pub jwt_token: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            url: "http://localhost:3000".to_string(),
            jwt_token: Default::default(),
        }
    }
}

fn main() {
    let mut state = State::default();
    loop {
        let cmd = Text::new("> ").with_validator(required!()).prompt();
        if cmd.is_err() {
            process::exit(1);
        }
        let cmd = cmd.unwrap();
        for existing_cmd in COMMANDS.iter() {
            if existing_cmd.name == cmd {
                let res = (existing_cmd.exec)(vec!["".to_string()], &mut state);
                if let Err(e) = res {
                    cprintln!("<bold,red>Error</bold,red>: {}", e);
                }
                break;
            }
        }
    }
}
