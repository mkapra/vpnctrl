use anyhow::Result;
use async_graphql::Context;
use async_graphql::InputObject;
use async_graphql::{ComplexObject, SimpleObject};
use libwgbuilder::models::Client as DbClient;
use libwgbuilder::models::Keypair as DbKeypair;
use libwgbuilder::models::Model;
use libwgbuilder::models::Server as DbServer;
use libwgbuilder::models::VpnIp as DbVpnIp;

use crate::{schema::get_db_connection, auth::ServerGuard};

use super::vpn_ip::NewVpnIp;
use super::Keypair;
use super::VpnIp;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Server {
    id: i32,
    name: String,
    description: Option<String>,
    forward_interface: Option<String>,
    external_ip: String,
}

impl From<DbServer> for Server {
    fn from(s: DbServer) -> Self {
        Server {
            id: s.id,
            name: s.name,
            description: s.description,
            forward_interface: s.forward_interface,
            external_ip: s.external_ip,
        }
    }
}

#[ComplexObject]
impl Server {
    async fn keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        let server = DbServer::find(self.id, &mut db)?;
        Ok(Keypair::from(DbKeypair::find(server.keypair_id, &mut db)?))
    }

    async fn vpn_ip(&self, ctx: &Context<'_>) -> Result<VpnIp> {
        let mut db = get_db_connection(ctx)?;
        let client = DbServer::find(self.id, &mut db)?;
        Ok(VpnIp::from(DbVpnIp::find(client.vpn_ip_id, &mut db)?))
    }

    #[graphql(guard = "ServerGuard::new(self.id)")]
    async fn configuration(&self, ctx: &Context<'_>) -> Result<String> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(self.id, &mut db)?;
        client.configuration(&mut db)
    }
}

#[derive(InputObject)]
pub struct NewServer {
    pub name: String,
    pub description: Option<String>,
    pub forward_interface: Option<String>,
    pub keypair_id: i32,
    pub external_ip: String,
    pub vpn_ip: NewVpnIp,
}
