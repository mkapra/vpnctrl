//! Server model
//!
//! The server is the gateway for the clients
use anyhow::Result;
use diesel::{
    prelude::*, Associations, Identifiable, Insertable, PgConnection, QueryDsl, Queryable,
    RunQueryDsl,
};
use sailfish::TemplateOnce;

use crate::{
    models::{AllowedIp, Client},
    schema::servers,
    Error,
};

use super::{Keypair, Model, VpnIp, VpnNetwork};

/// Server from the database
#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(Keypair))]
#[diesel(belongs_to(VpnIp))]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub forward_interface: Option<String>,
    pub keypair_id: i32,
    pub vpn_ip_id: i32,
    pub external_ip: String,
}

impl Model for Server {
    fn find(search_id: i32, conn: &mut PgConnection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        use crate::schema::servers::dsl::*;
        servers.find(search_id).first::<Self>(conn).map_err(|e| {
            anyhow::Error::from(e).context(Error::DatabaseObjectNotFound("server", search_id))
        })
    }
}

impl Server {
    /// Returns a wireguard configuration for the server
    pub fn configuration(&self, conn: &mut PgConnection) -> Result<String> {
        let vpn_ip = VpnIp::find(self.vpn_ip_id, conn)?;
        let vpn_network = VpnNetwork::find(vpn_ip.vpn_network_id, conn)?;
        let keypair = Keypair::find(self.keypair_id, conn)?;

        use crate::schema::{
            clients,
            vpn_ips::{self, vpn_network_id},
        };
        let clients: Vec<(Client, VpnIp)> = clients::table
            .inner_join(vpn_ips::table)
            .filter(vpn_network_id.eq(vpn_network.id))
            .load(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not create config for server")
            })?;
        let mut template_clients = Vec::with_capacity(clients.len());
        for (c, ip) in clients {
            let keypair = Keypair::find(c.keypair_id, conn)?;
            let allowed_ips = AllowedIp::get_sending_ips_for_client(&c, conn)?
                .into_iter()
                .map(|e| e.address)
                .collect::<Vec<String>>()
                .join(", ");

            template_clients.push(TemplateClient {
                name: c.name.clone(),
                public_key: keypair.public_key,
                ip: ip.address.clone(),
                sending_ips: allowed_ips,
            });
        }

        let ctx = ServerConfig {
            ip_address: vpn_ip.address,
            netmask: vpn_network.subnetmask,
            listen_port: vpn_network.port,
            private_key: keypair.private_key,
            clients: template_clients,
        };
        ctx.render_once().map_err(|e| {
            anyhow::Error::from(e).context("Could not create configuration for server")
        })
    }

    fn get_vpn_ip(&self, conn: &mut PgConnection) -> Result<VpnIp> {
        use crate::schema::{
            servers::{self, dsl::*},
            vpn_ips,
        };
        let (_, server_ip): (Self, VpnIp) = servers::table
            .filter(vpn_ip_id.eq(self.vpn_ip_id))
            .inner_join(vpn_ips::table)
            .first(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not get vpn ip address of server")
            })?;
        Ok(server_ip)
    }

    pub fn get_associated_clients(&self, conn: &mut PgConnection) -> Result<Vec<Client>> {
        use crate::schema::{
            clients,
            vpn_ips::{self, dsl::*},
        };

        let server_ip = self.get_vpn_ip(conn)?;

        let assoc_clients = clients::table
            .inner_join(vpn_ips::table)
            .filter(vpn_network_id.eq(server_ip.vpn_network_id))
            .load::<(Client, VpnIp)>(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not get clients associated with server")
            })?
            .into_iter()
            .map(|(c, _)| c)
            .collect::<Vec<Client>>();
        Ok(assoc_clients)
    }
}

/// Client representation for the server configuration template
struct TemplateClient {
    name: String,
    public_key: String,
    ip: String,
    sending_ips: String,
}

#[derive(TemplateOnce)]
#[template(path = "server_configuration.stpl")]
struct ServerConfig {
    ip_address: String,
    netmask: i32,
    listen_port: i32,
    private_key: String,
    clients: Vec<TemplateClient>,
}

/// Server that is not created in the database yet
#[derive(Insertable)]
#[diesel(table_name = servers)]
pub struct NewServer<'a> {
    name: &'a str,
    description: Option<String>,
    forward_interface: Option<String>,
    keypair_id: i32,
    vpn_ip_id: i32,
    external_ip: &'a str,
}

impl NewServer<'_> {
    /// Returns a new server that is ready to be inserted into the database
    pub fn new<'a>(
        name: &'a str,
        description: Option<String>,
        forward_interface: Option<String>,
        keypair_id: i32,
        vpn_ip_id: i32,
        external_ip: &'a str,
    ) -> NewServer<'a> {
        NewServer {
            name,
            description,
            forward_interface,
            keypair_id,
            vpn_ip_id,
            external_ip,
        }
    }

    /// Creates a new server in the database
    pub fn create(self, conn: &mut PgConnection) -> Result<Server> {
        diesel::insert_into(servers::table)
            .values(&self)
            .get_result(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not create server in database")
            })
    }
}
