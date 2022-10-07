//! VPN network model
//!
//! This model represents the network that is used for the VPN configuration. Each participant (client and server)
//! has an ip address in this network
use anyhow::Result;
use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl};

use crate::{schema::vpn_networks, Error};

/// VPN network from the database
#[derive(Queryable)]
pub struct VpnNetwork {
    pub id: i32,
    pub network: String,
    pub subnetmask: i32,
    pub interface: i32,
    pub port: i32,
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
