//! Queries in the GraphQL schema
use anyhow::Result;
use async_graphql::{Context, Object};
use libwgbuilder::models::Model;

use crate::{
    auth::{ClientGuard, ServerGuard, UserGuard, UserRole},
    models::{Client, DnsServer, Keypair, Server, VpnIp, VpnNetwork},
};

use super::get_db_connection;

/// The root of the query type
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the requested keypair
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn keypair(&self, ctx: &Context<'_>, id: i32) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        let keypair = libwgbuilder::models::Keypair::find(id, &mut db)?;
        Ok(Keypair::from(keypair))
    }

    /// Returns the requested dns server
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn dns_server(&self, ctx: &Context<'_>, id: i32) -> Result<DnsServer> {
        let mut db = get_db_connection(ctx)?;
        let dns_server = libwgbuilder::models::DnsServer::find(id, &mut db)?;
        Ok(DnsServer::from(dns_server))
    }

    /// Returns the requested VPN network
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn vpn_network(&self, ctx: &Context<'_>, id: i32) -> Result<VpnNetwork> {
        let mut db = get_db_connection(ctx)?;
        let vpn_network = libwgbuilder::models::VpnNetwork::find(id, &mut db)?;
        Ok(VpnNetwork::from(vpn_network))
    }

    /// Returns the requested VPN IP address
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn vpn_ip(&self, ctx: &Context<'_>, id: i32) -> Result<VpnIp> {
        let mut db = get_db_connection(ctx)?;
        let vpn_ip = libwgbuilder::models::VpnIp::find(id, &mut db)?;
        Ok(VpnIp::from(vpn_ip))
    }

    /// Returns the requested client
    #[graphql(guard = "UserGuard::new(UserRole::Admin).or(ClientGuard::new(id))")]
    async fn client(&self, ctx: &Context<'_>, id: i32) -> Result<Client> {
        let mut db = get_db_connection(ctx)?;
        let client = libwgbuilder::models::Client::find(id, &mut db)?;
        Ok(Client::from(client))
    }

    /// Returns the requested server
    #[graphql(guard = "UserGuard::new(UserRole::Admin).or(ServerGuard::new(id))")]
    async fn server(&self, ctx: &Context<'_>, id: i32) -> Result<Server> {
        let mut db = get_db_connection(ctx)?;
        let server = libwgbuilder::models::Server::find(id, &mut db)?;
        Ok(Server::from(server))
    }
}
