use anyhow::Result;
use async_graphql::{Context, Object};

use libwgbuilder::models::dns_server::NewDnsServer as NewDbDnsServer;
use libwgbuilder::models::keypair::NewKeypair as NewDbKeypair;

use super::get_db_connection;
use crate::models::{dns_server::NewDnsServer, DnsServer, Keypair};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn generate_keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        Ok(Keypair::from(NewDbKeypair::generate(&mut db)?))
    }

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
}
