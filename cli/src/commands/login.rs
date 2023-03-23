use inquire::{required, Password, Text};

use crate::{queries::Login, State};

type UserInput = (String, String);

fn get_user_data(ctx: State) -> (UserInput, State) {
    let api_url = Text::new("API URL:").prompt().unwrap();

    let mut state = State { ..ctx };
    if !api_url.is_empty() {
        state.url = api_url.clone();
    }

    let username = Text::new("Username:")
        .with_validator(required!())
        .prompt()
        .unwrap();
    let password = Password::new("Password:")
        .without_confirmation()
        .with_validator(required!())
        .prompt()
        .unwrap();
    ((username, password), state)
}

pub fn login(_args: Vec<String>, ctx: State) -> State {
    let ((username, password), mut state) = get_user_data(ctx);
    let token = Login::query(&state, username, password).unwrap();
    state.jwt_token = token;
    state
}
