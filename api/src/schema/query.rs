//! Queries in the GraphQL schema
use anyhow::Result;
use async_graphql::{Context, Object};
use libwgbuilder::models::Model;

use crate::models::{DnsServer, Keypair};

use super::get_db_connection;

/// The root of the query type
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the requested keypair
    async fn keypair(&self, ctx: &Context<'_>, id: i32) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        let keypair = libwgbuilder::models::Keypair::find(id, &mut db)?;
        Ok(Keypair::from(keypair))
    }

    /// Returns the requested dns server
    async fn dns_server(&self, ctx: &Context<'_>, id: i32) -> Result<DnsServer> {
        let mut db = get_db_connection(ctx)?;
        let dns_server = libwgbuilder::models::DnsServer::find(id, &mut db)?;
        Ok(DnsServer::from(dns_server))
    }
}
