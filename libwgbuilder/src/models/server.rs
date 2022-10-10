//! Server model
//!
//! The server is the gateway for the clients
use anyhow::Result;
use diesel::{
    Associations, Identifiable, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};

use crate::{schema::servers, Error};

use super::{Keypair, Model, VpnIp};

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
