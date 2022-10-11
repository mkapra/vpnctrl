//! Holds all the necessary parts for the schema
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::database::DatabaseConn;

mod query;
pub use query::QueryRoot;

/// GraphQL schema of the API
pub type WireguardSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Creates a schema and returns it
///
/// # Arguments
/// * `pool` - A connection to the database
pub fn build_schema(pool: DatabaseConn) -> WireguardSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .data(QueryRoot::new())
        .finish()
}
