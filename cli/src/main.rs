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
        let cmd = Text::new("> ")
            .with_validator(required!())
            .prompt()
            .expect("Could not get command from user");
        for existing_cmd in COMMANDS.iter() {
            if existing_cmd.name == cmd {
                state = (existing_cmd.exec)(vec!["".to_string()], state);
                break;
            }
        }
    }
}
