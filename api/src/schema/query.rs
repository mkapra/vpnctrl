//! Queries in the GraphQL schema
use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};
use libwgbuilder::models::Model;

use crate::database::{DatabaseConn, DatabaseConnection};
use crate::models::Keypair;

fn get_db_connection(ctx: &Context<'_>) -> Result<DatabaseConnection> {
    ctx.data::<DatabaseConn>()
        .map_err(|_| anyhow!("Could not get database connection from context"))?
        .get()
}

/// The root of the query type
pub struct QueryRoot;

impl QueryRoot {
    pub fn new() -> Self {
        QueryRoot {}
    }
}

#[Object]
impl QueryRoot {
    /// Returns the requested keypair
    async fn keypair(&self, ctx: &Context<'_>, id: i32) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        let keypair = libwgbuilder::models::Keypair::find(id, &mut db)?;
        Ok(Keypair::from(keypair))
    }
}
