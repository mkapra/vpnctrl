use anyhow::Result;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use libwgbuilder::models::{
    AllowedIp, Client as DbClient, DnsServer as DbDnsServer, Keypair as DbKeypair, Model,
    VpnIp as DbVpnIp,
};

use crate::schema::get_db_connection;

use super::{vpn_ip::NewVpnIp, DnsServer, Keypair, Server, VpnIp};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Client {
    id: i32,
    name: String,
    description: Option<String>,
    keepalive: i32,
}

impl From<DbClient> for Client {
    fn from(c: DbClient) -> Self {
        Client {
            id: c.id,
            name: c.name,
            description: c.description,
            keepalive: c.keepalive,
        }
    }
}

#[ComplexObject]
impl Client {
    async fn dns_server(&self, ctx: &Context<'_>) -> Result<Option<DnsServer>> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(self.id, &mut db)?;

        if client.dns_server_id.is_none() {
            return Ok(None);
        }

        Ok(Some(DnsServer::from(DbDnsServer::find(
            client.dns_server_id.unwrap(),
            &mut db,
        )?)))
    }

    async fn keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(self.id, &mut db)?;
        Ok(Keypair::from(DbKeypair::find(client.keypair_id, &mut db)?))
    }

    async fn vpn_ip(&self, ctx: &Context<'_>) -> Result<VpnIp> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(self.id, &mut db)?;
        Ok(VpnIp::from(DbVpnIp::find(client.vpn_ip_id, &mut db)?))
    }

    async fn configuration(&self, ctx: &Context<'_>) -> Result<String> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(self.id, &mut db)?;
        client.configuration(&mut db)
    }

    async fn allowed_ips(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(self.id, &mut db)?;
        AllowedIp::get_allowed_ips_for_client(&client, &mut db)
            .map(|v| v.into_iter().map(|a| a.address).collect())
    }

    async fn allowed_ips_sending(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(self.id, &mut db)?;
        AllowedIp::get_sending_ips_for_client(&client, &mut db)
            .map(|v| v.into_iter().map(|a| a.address).collect())
    }

    async fn server(&self, ctx: &Context<'_>) -> Result<Server> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(self.id, &mut db)?;
        client
            .get_associated_server(&mut db)
            .map(|s| Server::from(s))
    }
}

#[derive(InputObject)]
pub struct NewClient {
    pub name: String,
    pub description: Option<String>,
    pub dns_server_id: Option<i32>,
    pub keepalive: i32,
    pub vpn_ip: NewVpnIp,
}
