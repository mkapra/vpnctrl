use anyhow::Error;
use anyhow::Result;
use reqwest::blocking::Client;

pub use crate::State;

mod new;
pub use new::*;
mod login;
pub use login::Login;

/// Builds a reqwest `Client` with the authorization token from the [`State`]
pub fn build_client(ctx: &State) -> Result<Client> {
    Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", ctx.jwt_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()
        .map_err(Error::from)
}
