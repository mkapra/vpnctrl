//! VPN network model
//!
//! This model represents the network that is used for the VPN configuration. Each participant (client and server)
//! has an ip address in this network
use anyhow::Result;
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};

use crate::{schema::vpn_networks, Error};

use super::Model;

/// VPN network from the database
#[derive(Queryable)]
pub struct VpnNetwork {
    pub id: i32,
    pub network: String,
    pub subnetmask: i32,
    pub interface: i32,
    pub port: i32,
}

impl Model for VpnNetwork {
    fn find(search_id: i32, conn: &mut PgConnection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        use crate::schema::vpn_networks::dsl::*;
        vpn_networks
            .find(search_id)
            .first::<Self>(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not find VPN network")
            })
    }
}

/// VPN network that is not created in the database yet
#[derive(Insertable)]
#[diesel(table_name = vpn_networks)]
pub struct NewVpnNetwork<'a> {
    network: &'a str,
    subnetmask: i32,
    interface: i32,
    port: i32,
}

impl NewVpnNetwork<'_> {
    /// Returns a new VPN network that is ready to be inserted into the database
    pub fn new<'a>(
        network: &'a str,
        subnetmask: i32,
        interface: i32,
        port: i32,
    ) -> NewVpnNetwork<'a> {
        NewVpnNetwork {
            network,
            subnetmask,
            interface,
            port,
        }
    }

    /// Creates a new VPN network in the database
    pub fn create(self, conn: &mut PgConnection) -> Result<VpnNetwork> {
        diesel::insert_into(vpn_networks::table)
            .values(&self)
            .get_result(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not create keypair in database")
            })
    }
}
