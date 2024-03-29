//! DNS server model
//!
//! DNS servers are used in the configs for the clients. This config option will set up the DNS server
//! used by the client while connected to the VPN.
use anyhow::Result;
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};

use crate::{schema::dns_servers, Error};

use super::Model;

/// DNS server from the database
#[derive(Queryable)]
pub struct DnsServer {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub description: Option<String>,
}

impl DnsServer {
    pub fn all(conn: &mut PgConnection) -> anyhow::Result<Vec<Self>> {
        use crate::schema::dns_servers::dsl::*;
        dns_servers
            .load(conn)
            .map_err(|e| anyhow::Error::from(e).context("Could not load DNS servers"))
    }
}

impl Model for DnsServer {
    fn find(search_id: i32, conn: &mut PgConnection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        use crate::schema::dns_servers::dsl::*;
        dns_servers
            .find(search_id)
            .first::<Self>(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::DatabaseObjectNotFound("DNS server", search_id))
            })
    }
}

/// DNS server that is not created in the database yet
#[derive(Insertable)]
#[diesel(table_name = dns_servers)]
pub struct NewDnsServer<'a> {
    name: &'a str,
    ip: &'a str,
    description: Option<String>,
}

impl NewDnsServer<'_> {
    /// Returns a new DNS server that is ready to be inserted into the database
    pub fn new<'a>(name: &'a str, ip: &'a str, description: Option<String>) -> NewDnsServer<'a> {
        NewDnsServer {
            name,
            ip,
            description,
        }
    }

    /// Creates a new DNS server in the database
    pub fn create(self, conn: &mut PgConnection) -> Result<DnsServer> {
        diesel::insert_into(dns_servers::table)
            .values(&self)
            .get_result(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not create DNS server in database")
            })
    }
}
