use inquire::{required, Text};
use once_cell::sync::Lazy;

mod commands;
use commands::*;
mod queries;

#[derive(Debug)]
pub struct State {
    pub url: String,
    pub jwt_token: String,
}

impl State {
    fn new(url: Option<&str>) -> Self {
        Self {
            jwt_token: "".to_string(),
            url: url
                .map(|u| u.to_string())
                .unwrap_or_else(|| "http://localhost:3000".to_string()),
        }
    }
}

static COMMANDS: Lazy<Vec<Command<fn(Vec<String>, State) -> State>>> = Lazy::new(|| {
    vec![
        Command::new("help", "Prints out this help", print_help),
        Command::new(
            "login",
            "Is required to retrieve a token from the API",
            login,
        ),
        Command::new(
            "new_client",
            "Creates a new client in the network",
            new_client,
        ),
    ]
});

struct Command<C>
where
    C: Fn(Vec<String>, State) -> State,
{
    name: String,
    exec: C,
    help: String,
}

impl<C> Command<C>
where
    C: Fn(Vec<String>, State) -> State,
{
    fn new(name: &str, help: &str, exec: C) -> Self {
        Self {
            name: name.to_string(),
            help: help.to_string(),
            exec,
        }
    }
}

fn main() {
    let mut state = State::new(None);
    loop {
        let cmd = Text::new("> ")
            .with_validator(required!())
            .prompt()
            .unwrap();
        for existing_cmd in COMMANDS.iter() {
            if existing_cmd.name == cmd {
                state = (existing_cmd.exec)(vec!["".to_string()], state);
                break;
            }
        }
    }
}
