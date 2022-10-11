//! Tokens for authentication of client and server
//!
//! The tokens are used for the clients that are used by server and client to retrieve
//! their VPN configuration
use anyhow::Result;
use diesel::{prelude::*, Insertable, PgConnection, Queryable, result::Error::NotFound};
use uuid::Uuid;

use crate::{
    models::{Model, Server},
    schema::{tokens_clients, tokens_servers},
    Error,
};

use super::Client;

#[derive(Queryable)]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub expires_at: Option<chrono::NaiveDateTime>,
}

impl Token {
    /// Assigns a token to the given client
    pub fn assign_to_client(&self, client_id: i32, conn: &mut PgConnection) -> Result<()> {
        let new_client = NewClientToken {
            token_id: self.id,
            client_id,
        };
        diesel::insert_into(tokens_clients::table)
            .values(&new_client)
            .execute(conn)
            .map(|_| ())
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not assign token to client in database")
            })
    }

    /// Assigns a token to the given server
    pub fn assign_to_server(&self, server_id: i32, conn: &mut PgConnection) -> Result<()> {
        let new_server = NewServerToken {
            token_id: self.id,
            server_id,
        };
        diesel::insert_into(tokens_servers::table)
            .values(&new_server)
            .execute(conn)
            .map(|_| ())
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not assign token to server in database")
            })
    }

    /// Returns the client for the given token
    ///
    /// # Returns
    /// Returns an `Err` if no client for the token was found
    pub fn get_client_for_token(user_token: &str, conn: &mut PgConnection) -> Result<Client> {
        use crate::schema::tokens::{self, dsl::*};
        let (_, (_, client_id)) = tokens::table
            .inner_join(tokens_clients::table)
            .filter(token.eq(user_token))
            .first::<(Token, (i32, i32))>(conn)
            .map_err(|e| match e {
                NotFound => Error::DatabaseObjectNotFound("token", 0),
                _ => Error::Database,
            })?;
        Client::find(client_id, conn)
    }

    /// Returns the server for the given token
    ///
    /// # Returns
    /// Returns an `Err` if no server for the token was found
    pub fn get_server_for_token(user_token: &str, conn: &mut PgConnection) -> Result<Server> {
        use crate::schema::tokens::{self, dsl::*};
        let (_, (_, server_id)) = tokens::table
            .inner_join(tokens_servers::table)
            .filter(token.eq(user_token))
            .first::<(Token, (i32, i32))>(conn)
            .map_err(|e| match e {
                NotFound => Error::DatabaseObjectNotFound("token", 0),
                _ => Error::Database,
            })?;
        Server::find(server_id, conn)
    }
}

#[derive(Insertable)]
#[diesel(table_name = tokens_clients)]
struct NewClientToken {
    token_id: i32,
    client_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tokens_servers)]
struct NewServerToken {
    token_id: i32,
    server_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tokens)]
pub struct NewToken<'a> {
    pub token: &'a str,
    pub expires_at: Option<chrono::NaiveDateTime>,
}

impl NewToken<'_> {
    /// Returns a new token that is ready to be inserted into the database
    fn new(token: &str) -> NewToken {
        NewToken {
            token,
            expires_at: None,
        }
    }

    /// Generates a new random token and creates it in the database
    pub fn generate(conn: &mut PgConnection) -> Result<Token> {
        NewToken::new(&Uuid::new_v4().to_string()).create(conn)
    }

    /// Creates a token in the database
    pub fn create(self, conn: &mut PgConnection) -> Result<Token> {
        use crate::schema::tokens;
        diesel::insert_into(tokens::table)
            .values(&self)
            .get_result(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not create token in database")
            })
    }
}
