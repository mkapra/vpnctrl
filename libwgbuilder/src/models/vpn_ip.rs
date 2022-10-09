//! VPN IP address
//!
//! Each client and server in the VPN network needs an IP address.
use anyhow::Result;
use diesel::{
    Associations, Identifiable, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};

use crate::{schema::vpn_ips, Error};

use super::{Model, VpnNetwork};

/// VPN IP from the database
#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(VpnNetwork))]
pub struct VpnIp {
    pub id: i32,
    pub address: String,
    pub vpn_network_id: i32,
}

impl Model for VpnIp {
    fn find(search_id: i32, conn: &mut PgConnection) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        use crate::schema::vpn_ips::dsl::*;
        vpn_ips.find(search_id).first::<Self>(conn).map_err(|e| {
            anyhow::Error::from(e)
                .context(Error::DatabaseObjectNotFound("VPN ip address", search_id))
        })
    }
}

/// A VPN IP address that is not created in the database yet
#[derive(Insertable)]
#[diesel(table_name = vpn_ips)]
pub struct NewVpnIp<'a> {
    address: &'a str,
    vpn_network_id: i32,
}

impl NewVpnIp<'_> {
    /// Returns a new VPN IP address object that is ready for inserting into the database
    pub fn new(address: &str, vpn_network_id: i32) -> NewVpnIp {
        NewVpnIp {
            address,
            vpn_network_id,
        }
    }

    /// Creates a new VPN IP address in the database
    pub fn create(self, conn: &mut PgConnection) -> Result<VpnIp> {
        diesel::insert_into(vpn_ips::table)
            .values(&self)
            .get_result(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not create VPN IP address in database")
            })
    }
}
