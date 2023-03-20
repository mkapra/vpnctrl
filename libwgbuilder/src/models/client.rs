//! Client model
//!
//! A client is a device in the VPN network.
use anyhow::Result;
use diesel::{
    prelude::*, Associations, Identifiable, Insertable, PgConnection, QueryDsl, Queryable,
    RunQueryDsl,
};
use sailfish::TemplateOnce;

use crate::{
    models::{AllowedIp, Server},
    schema::clients,
    Error,
};

use super::{DnsServer, Keypair, Model, VpnIp, VpnNetwork};

/// Client from the database
#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(DnsServer))]
#[diesel(belongs_to(Keypair))]
#[diesel(belongs_to(VpnIp))]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub dns_server_id: Option<i32>,
    pub keepalive: i32,
    pub keypair_id: i32,
    pub vpn_ip_id: i32,
}

impl Model for Client {
    fn find(search_id: i32, conn: &mut PgConnection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        use crate::schema::clients::dsl::*;
        clients.find(search_id).first::<Self>(conn).map_err(|e| {
            anyhow::Error::from(e).context(Error::DatabaseObjectNotFound("client", search_id))
        })
    }
}

impl Client {
    /// Returns a wireguard configuration for the client
    ///
    /// # Returns
    /// If the configuration has missing parts (e.g. no server in VPN network) an `Err` will be returned
    pub fn configuration(&self, conn: &mut PgConnection) -> Result<String> {
        let keypair = Keypair::find(self.keypair_id, conn)?;
        let vpn_ip = VpnIp::find(self.vpn_ip_id, conn)?;
        let vpn_network = VpnNetwork::find(vpn_ip.vpn_network_id, conn)?;
        let dns_server_ip = if let Some(dns_id) = self.dns_server_id {
            DnsServer::find(dns_id, conn)?.ip
        } else {
            "".to_string()
        };

        // Find the corresponding server in the VPN network of the client
        use crate::schema::vpn_ips::{self, dsl::*};
        let ids_ip_network = vpn_ips::table
            .filter(vpn_network_id.eq(vpn_network.id))
            .load::<VpnIp>(conn)?
            .iter()
            .map(|e| e.id)
            .collect::<Vec<i32>>();
        use crate::schema::servers::{self, dsl::*};
        let (server, _): (Server, VpnIp) = servers::table
            .inner_join(vpn_ips::table)
            .filter(vpn_ip_id.eq_any(ids_ip_network))
            .first(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not get server from database")
            })?;
        let server_keypair = Keypair::find(server.keypair_id, conn)?;
        let allowed_ips = AllowedIp::get_allowed_ips_for_client(self, conn)?
            .into_iter()
            .map(|e| e.address)
            .collect::<Vec<String>>()
            .join(", ");

        let ctx = ClientConfig {
            private_key: keypair.private_key,
            ip_address: vpn_ip.address,
            netmask: vpn_network.subnetmask,
            dns_server: dns_server_ip,
            peer_public_key: server_keypair.public_key,
            endpoint_address: server.external_ip,
            endpoint_port: vpn_network.port,
            keepalive: self.keepalive,
            allowed_ips: match allowed_ips.is_empty() {
                true => "0.0.0.0/0".to_string(),
                false => allowed_ips,
            },
        };
        ctx.render_once().map_err(|e| {
            anyhow::Error::from(e).context("Could not create configuration for client")
        })
    }
}

#[derive(TemplateOnce)]
#[template(path = "client_configuration.stpl")]
struct ClientConfig {
    private_key: String,
    ip_address: String,
    netmask: i32,
    dns_server: String,
    peer_public_key: String,
    endpoint_address: String,
    endpoint_port: i32,
    keepalive: i32,
    allowed_ips: String,
}

/// Client that is not created in the database yet
#[derive(Insertable)]
#[diesel(table_name = clients)]
pub struct NewClient<'a> {
    name: &'a str,
    description: Option<String>,
    dns_server_id: Option<i32>,
    keepalive: i32,
    keypair_id: i32,
    vpn_ip_id: i32,
}

impl NewClient<'_> {
    /// Returns a new client that is ready to be inserted into the database
    pub fn new(
        name: &str,
        description: Option<String>,
        dns_server_id: Option<i32>,
        keepalive: i32,
        keypair_id: i32,
        vpn_ip_id: i32,
    ) -> NewClient {
        NewClient {
            name,
            description,
            dns_server_id,
            keepalive,
            keypair_id,
            vpn_ip_id,
        }
    }

    /// Creates a new client in the database
    pub fn create(self, conn: &mut PgConnection) -> Result<Client> {
        diesel::insert_into(clients::table)
            .values(&self)
            .get_result(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not create client in database")
            })
    }
}
