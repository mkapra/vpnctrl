use anyhow::Result;
use async_graphql::{Context, Object};

use libwgbuilder::models::dns_server::NewDnsServer as NewDbDnsServer;
use libwgbuilder::models::keypair::NewKeypair as NewDbKeypair;
use libwgbuilder::models::vpn_network::NewVpnNetwork as NewDbVpnNetwork;

use super::get_db_connection;
use crate::models::{
    dns_server::NewDnsServer, vpn_network::NewVpnNetwork, DnsServer, Keypair, VpnNetwork,
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
}
