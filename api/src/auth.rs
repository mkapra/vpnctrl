use anyhow::anyhow;
use async_graphql::{async_trait, Context, Guard, Result};
use libwgbuilder::models::Token;
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};

use crate::schema::get_db_connection;

#[derive(Debug, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Client,
}

pub struct UserGuard {
    role: UserRole,
}

impl UserGuard {
    pub fn new(role: UserRole) -> Self {
        UserGuard { role }
    }
}

#[async_trait::async_trait]
impl Guard for UserGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let token = ctx
            .data::<ApiKey>()
            .map_err(|_| anyhow!("Could not get api token from user"))?;
        if token.0 == "test123" && self.role == UserRole::Admin {
            return Ok(());
        }
        Err(async_graphql::Error::new("Invalid user token"))
    }
}

/// This guard protects the configuration attribute of the client
pub struct ClientGuard {
    id: i32,
}

impl ClientGuard {
    pub fn new(id: i32) -> Self {
        ClientGuard { id }
    }
}

#[async_trait::async_trait]
impl Guard for ClientGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let mut db = get_db_connection(ctx)?;
        let token = ctx
            .data::<ApiKey>()
            .map_err(|_| anyhow!("Could not get api token from user"))?;
        if let Ok(c) = Token::get_client_for_token(&token.0, &mut db) {
            if c.id != self.id {
                return Err(async_graphql::Error::new("Invalid token"));
            }
            return Ok(());
        }

        Err(async_graphql::Error::new("Invalid token"))
    }
}

pub struct ServerGuard {
    id: i32,
}

impl ServerGuard {
    pub fn new(id: i32) -> Self {
        ServerGuard { id }
    }
}

#[async_trait::async_trait]
impl Guard for ServerGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let mut db = get_db_connection(ctx)?;
        let token = ctx
            .data::<ApiKey>()
            .map_err(|_| anyhow!("Could not get api token from user"))?;
        if let Ok(s) = Token::get_server_for_token(&token.0, &mut db) {
            if s.id != self.id {
                return Err(async_graphql::Error::new("Invalid token"));
            }
            return Ok(());
        }

        Err(async_graphql::Error::new("Invalid token"))
    }
}

/// Holds the API key from the `Token` header
#[derive(Debug)]
pub struct ApiKey(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Token") {
            Some(token) => Outcome::Success(ApiKey(token.to_string())),
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
