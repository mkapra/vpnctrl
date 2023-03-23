use anyhow::{Error, Result};
use inquire::{required, Password, Text};

use crate::{queries::Login, State};

type UserInput = (String, String);

fn get_user_data(ctx: &mut State) -> Result<UserInput> {
    let api_url = Text::new("API URL:").prompt().unwrap();
    if !api_url.is_empty() {
        ctx.url = api_url.clone();
    }

    let username = Text::new("Username:")
        .with_validator(required!())
        .prompt()
        .map_err(|e| Error::from(e).context("Could not get username"))?;
    let password = Password::new("Password:")
        .without_confirmation()
        .with_validator(required!())
        .prompt()
        .map_err(|e| Error::from(e).context("Could not get password"))?;
    Ok((username, password))
}

pub fn login(_args: Vec<String>, ctx: &mut State) -> Result<()> {
    let (username, password) = get_user_data(ctx)?;
    let token = Login::query(&ctx, username, password);
    ctx.jwt_token = token?;
    Ok(())
}
