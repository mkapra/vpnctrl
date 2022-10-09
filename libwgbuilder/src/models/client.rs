//! Client model
//!
//! A client is a device in the VPN network.
use anyhow::Result;
use diesel::{
    Associations, Identifiable, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};

use crate::{schema::clients, Error};

use super::{DnsServer, Keypair, Model, VpnIp};

/// Client from the database
#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(DnsServer))]
#[diesel(belongs_to(Keypair))]
#[diesel(belongs_to(VpnIp))]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub dns_server_id: i32,
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

/// Client that is not created in the database yet
#[derive(Insertable)]
#[diesel(table_name = clients)]
pub struct NewClient<'a> {
    name: &'a str,
    description: Option<String>,
    dns_server_id: i32,
    keepalive: i32,
    keypair_id: i32,
    vpn_ip_id: i32,
}

impl NewClient<'_> {
    /// Returns a new client that is ready to be inserted into the database
    pub fn new(
        name: &str,
        description: Option<String>,
        dns_server_id: i32,
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
