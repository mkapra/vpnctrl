use anyhow::Result;
use async_graphql::{Context, Object};

use libwgbuilder::models::vpn_ip::NewVpnIp;
use libwgbuilder::models::{client::NewClient as NewDbClient};
use libwgbuilder::models::dns_server::NewDnsServer as NewDbDnsServer;
use libwgbuilder::models::keypair::NewKeypair as NewDbKeypair;
use libwgbuilder::models::vpn_network::NewVpnNetwork as NewDbVpnNetwork;

use super::get_db_connection;
use crate::models::{
    client::NewClient, dns_server::NewDnsServer, vpn_network::NewVpnNetwork, Client, DnsServer,
    Keypair, VpnNetwork,
};

pub struct Mutation;

#[Object]
impl Mutation {
    /// Generates a keypair
    async fn generate_keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        Ok(Keypair::from(NewDbKeypair::generate(&mut db)?))
    }

    /// Creates a DNS Server
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
    async fn new_client(&self, ctx: &Context<'_>, client: NewClient) -> Result<Client> {
        let mut db = get_db_connection(ctx)?;

        // Create VPN IP address
        let ip = NewVpnIp::new(&client.vpn_ip.address, client.vpn_ip.vpn_network_id).create(&mut db)?;

        Ok(Client::from(
            NewDbClient::new(
                &client.name,
                client.description,
                client.dns_server_id,
                client.keepalive,
                client.keypair_id,
                ip.id
            )
            .create(&mut db)?,
        ))
    }
}
