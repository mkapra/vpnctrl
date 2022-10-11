use anyhow::Result;
use async_graphql::{Context, Object};

use libwgbuilder::models::client::NewClient as NewDbClient;
use libwgbuilder::models::dns_server::NewDnsServer as NewDbDnsServer;
use libwgbuilder::models::keypair::NewKeypair as NewDbKeypair;
use libwgbuilder::models::server::NewServer as NewDbServer;
use libwgbuilder::models::token::NewToken;
use libwgbuilder::models::vpn_ip::NewVpnIp;
use libwgbuilder::models::vpn_network::NewVpnNetwork as NewDbVpnNetwork;

use super::get_db_connection;
use crate::auth::{UserGuard, UserRole};
use crate::models::server::NewServer;
use crate::models::Server;
use crate::models::{
    client::NewClient, dns_server::NewDnsServer, vpn_network::NewVpnNetwork, Client, DnsServer,
    Keypair, VpnNetwork,
};

pub struct Mutation;

#[Object]
impl Mutation {
    /// Generates a keypair
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn generate_keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        Ok(Keypair::from(NewDbKeypair::generate(&mut db)?))
    }

    /// Creates a DNS Server
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn new_dns_server(
        &self,
        ctx: &Context<'_>,
        dns_server: NewDnsServer,
    ) -> Result<DnsServer> {
        let mut db = get_db_connection(ctx)?;
        Ok(DnsServer::from(
            NewDbDnsServer::new(&dns_server.name, &dns_server.ip, dns_server.description)
                .create(&mut db)?,
        ))
    }

    /// Creates a VPN network
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn new_vpn_network(
        &self,
        ctx: &Context<'_>,
        vpn_network: NewVpnNetwork,
    ) -> Result<VpnNetwork> {
        let mut db = get_db_connection(ctx)?;
        Ok(VpnNetwork::from(
            NewDbVpnNetwork::new(
                &vpn_network.network,
                vpn_network.subnetmask,
                vpn_network.interface,
                vpn_network.port,
            )
            .create(&mut db)?,
        ))
    }

    /// Creates a client
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn new_client(&self, ctx: &Context<'_>, client: NewClient) -> Result<Client> {
        let mut db = get_db_connection(ctx)?;

        // Create VPN IP address
        let ip =
            NewVpnIp::new(&client.vpn_ip.address, client.vpn_ip.vpn_network_id).create(&mut db)?;

        Ok(Client::from(
            NewDbClient::new(
                &client.name,
                client.description,
                client.dns_server_id,
                client.keepalive,
                client.keypair_id,
                ip.id,
            )
            .create(&mut db)?,
        ))
    }

    /// Creates a server
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn new_server(&self, ctx: &Context<'_>, server: NewServer) -> Result<Server> {
        let mut db = get_db_connection(ctx)?;

        // Create VPN IP address
        let ip =
            NewVpnIp::new(&server.vpn_ip.address, server.vpn_ip.vpn_network_id).create(&mut db)?;

        Ok(Server::from(
            NewDbServer::new(
                &server.name,
                server.description,
                server.forward_interface,
                server.keypair_id,
                ip.id,
                &server.external_ip,
            )
            .create(&mut db)?,
        ))
    }

    /// Assign a token to the given server
    ///
    /// This token can be used to retrieve the configuration for this particular server
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn generate_token_for_server(&self, ctx: &Context<'_>, server_id: i32) -> Result<String> {
        let mut db = get_db_connection(ctx)?;

        let token = NewToken::generate(&mut db)?;
        token.assign_to_server(server_id, &mut db)?;

        Ok(token.token)
    }

    /// Assign a token to the given client
    ///
    /// This token can be used to retrieve the configuration for this particular client
    #[graphql(guard = "UserGuard::new(UserRole::Admin)")]
    async fn generate_token_for_client(&self, ctx: &Context<'_>, client_id: i32) -> Result<String> {
        let mut db = get_db_connection(ctx)?;

        let token = NewToken::generate(&mut db)?;
        token.assign_to_client(client_id, &mut db)?;

        Ok(token.token)
    }
}
