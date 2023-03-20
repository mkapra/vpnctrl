use anyhow::anyhow;
use async_graphql::{async_trait, Context, Guard, Result};
use libwgbuilder::models::Token;
use poem::{http::StatusCode, FromRequest, Request, RequestBody};

use crate::schema::get_db_connection;

/// This guard protects all endpoints that should only be accessible by an administrator
pub struct AdminGuard;

impl AdminGuard {
    pub fn new() -> Self {
        AdminGuard {}
    }
}

#[async_trait::async_trait]
impl Guard for AdminGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let token = ctx
            .data::<ApiKey>()
            .map_err(|_| anyhow!("Could not get api key from user"))?;
        let secret = ctx
            .data::<jwt::Secret>()
            .map_err(|_| anyhow!("Could not get secret key"))?;

        if jwt::valid_token(&token.0, &secret.0) {
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

#[poem::async_trait]
impl<'a> FromRequest<'a> for ApiKey {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                poem::Error::from_string("{\"error\": \"missing token\"}", StatusCode::BAD_REQUEST)
            })?;

        Ok(ApiKey(
            token.to_string().split(' ').last().unwrap().to_string(),
        ))
    }
}

pub mod jwt {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};

    /// Used to store the secret for enrcypting/signing the JWT token
    pub struct Secret(pub String);

    impl Secret {
        pub fn new(secret: &str) -> Self {
            Secret(secret.to_owned())
        }
    }

    /// Claims for a JWT
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        pub sub: String,
        pub role: i32,
        pub exp: usize,
    }

    impl Claims {
        /// Creates a new claim with a expiration time of 10 minutes
        pub fn new(username: String, role: i32) -> Self {
            let expiration_time = Utc::now()
                .checked_add_signed(Duration::minutes(10))
                .expect("invalid timestamp")
                .timestamp();
            Claims {
                sub: username,
                role,
                exp: expiration_time as usize,
            }
        }
    }

    /// Generates a JWT for the given claims
    pub fn encode_jwt(claims: &Claims, secret: &str) -> anyhow::Result<String> {
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| anyhow::Error::from(e).context("Could not generate token for user"))
    }

    /// Validates a JWT token
    pub fn valid_token(token: &str, secret: &str) -> bool {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::default()),
        )
        .map(|_| true)
        .unwrap_or_else(|_| false)
    }
}
