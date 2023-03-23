//! Module with all the commands according with their information gathering
use once_cell::sync::Lazy;

use crate::State;

mod login;
pub use login::login;
mod new;
use new::new_client;

type CmdFunction = fn(Vec<String>, State) -> State;

/// Prints out the help for a [`Command`]
pub fn print_help(_args: Vec<String>, ctx: State) -> State {
    for cmd in COMMANDS.iter() {
        println!("{}:\t{}", cmd.name, cmd.help);
    }
    ctx
}

/// All available commands
pub static COMMANDS: Lazy<Vec<Command<CmdFunction>>> = Lazy::new(|| {
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

pub struct Command<C>
where
    C: Fn(Vec<String>, State) -> State,
{
    /// The name of a command is called by the user
    pub name: String,
    /// Function that gets executed when the user called the command
    pub exec: C,
    /// Help string that explains the command
    pub help: String,
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
