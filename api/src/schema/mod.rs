//! Holds all the necessary parts for the schema
use anyhow::{anyhow, Result};
use async_graphql::{Context, EmptySubscription, Schema};

use crate::database::{DatabaseConn, DatabaseConnection};

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
pub fn build_schema(pool: DatabaseConn) -> WireguardSchema {
    Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(pool)
        .finish()
}
