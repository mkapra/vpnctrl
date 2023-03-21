use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use inquire::{required, Password, Text};
use reqwest::blocking::Client;

use crate::{
    queries::{login, Login, NewClientInformation},
    State, COMMANDS,
};

pub fn print_help(_args: Vec<String>, ctx: State) -> State {
    for cmd in COMMANDS.iter() {
        println!("{}:\t{}", cmd.name, cmd.help);
    }
    ctx
}

pub fn login(_args: Vec<String>, ctx: State) -> State {
    let api_url = Text::new("API URL:").prompt().unwrap();
    let username = Text::new("Username:")
        .with_validator(required!())
        .prompt()
        .unwrap();
    let password = Password::new("Password:")
        .without_confirmation()
        .with_validator(required!())
        .prompt()
        .unwrap();

    let client = Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str("Bearer YESYOUFOUNDABUG").unwrap(),
            ))
            .collect(),
        )
        .build()
        .unwrap();

    let variables = login::Variables { username, password };

    let mut state = State { ..ctx };
    if !api_url.is_empty() {
        state.url = api_url;
    }

    let res = post_graphql::<Login, _>(&client, &state.url, variables).unwrap();
    let res_data = res.data.expect("Missing response data");
    state.jwt_token = res_data.login;
    state
}

pub fn new_client(_args: Vec<String>, ctx: State) -> State {
    NewClientInformation::create_client(&ctx).unwrap();
    ctx
}
