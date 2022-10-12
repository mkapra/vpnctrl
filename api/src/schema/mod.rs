//! Holds all the necessary parts for the schema
use anyhow::{anyhow, Result};
use async_graphql::{Context, EmptySubscription, Schema};

use crate::{
    auth::jwt::Secret,
    database::{DatabaseConn, DatabaseConnection},
};

mod query;
use query::QueryRoot;
mod mutation;
use mutation::Mutation;

/// GraphQL schema of the API
pub type WireguardSchema = Schema<QueryRoot, Mutation, EmptySubscription>;

/// Returns a connection from the database pool
///
/// The pool is located in the context from async-graphql
pub fn get_db_connection(ctx: &Context<'_>) -> Result<DatabaseConnection> {
    ctx.data::<DatabaseConn>()
        .map_err(|_| anyhow!("Could not get database connection from context"))?
        .get()
}

/// Creates a schema and returns it
///
/// # Arguments
/// * `pool` - A connection to the database
/// * `secret` - The secret is used to encryt/decrypt/sign/verify the JWT tokens
pub fn build_schema(pool: DatabaseConn, secret: Secret) -> WireguardSchema {
    Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(pool)
        .data(secret)
        .finish()
}
