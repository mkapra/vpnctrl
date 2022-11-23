//! Allowed IP addresses are used to restrict the traffic into the tunnel.
use diesel::prelude::*;

use super::{Client, Model};

/// Allowed IP from the database
#[derive(Queryable)]
pub struct AllowedIp {
    pub id: i32,
    pub address: String,
}

impl Model for AllowedIp {
    fn find(search_id: i32, conn: &mut PgConnection) -> anyhow::Result<Self> {
        use crate::schema::allowed_ips::dsl::*;
        allowed_ips
            .find(search_id)
            .first::<AllowedIp>(conn)
            .map_err(|e| anyhow::Error::from(e).context("Could not find allowed IP"))
    }
}

impl AllowedIp {
    /// This function checks if the given address already exists in the database and returns its id
    pub fn address_exists(requested_address: &str, conn: &mut PgConnection) -> anyhow::Result<i32> {
        use crate::schema::allowed_ips::dsl::*;
        allowed_ips
            .filter(address.eq(requested_address))
            .select(id)
            .first::<i32>(conn)
            .map_err(|e| anyhow::Error::from(e).context("Could not find allowed IP"))
    }

    /// This function assigns the allowed IP address to the given client
    ///
    /// Creates the ip address if it does not exist yet
    pub fn assign_ip_to_client(
        client: &Client,
        ip_address: &str,
        conn: &mut PgConnection,
    ) -> anyhow::Result<()> {
        use crate::schema::allowed_ips::dsl::*;
        use crate::schema::allowed_ips_clients::dsl::*;

        let address_id = match AllowedIp::address_exists(ip_address, conn) {
            Ok(i) => i,
            Err(_) => diesel::insert_into(allowed_ips)
                .values(address.eq(ip_address))
                .get_result::<AllowedIp>(conn)
                .map(|ip| ip.id)
                .map_err(|e| anyhow::Error::from(e).context("Could not insert allowed IP"))?,
        };

        diesel::insert_into(allowed_ips_clients)
            .values((client_id.eq(client.id), allowed_ip_id.eq(address_id)))
            .execute(conn)
            .map(|_| ())
            .map_err(|e| anyhow::Error::from(e).context("Could not insert allowed IP"))
    }

    /// Returns all the allowed IP addresses assigned to the given client
    pub fn get_allowed_ips_for_client(
        client: &Client,
        conn: &mut PgConnection,
    ) -> anyhow::Result<Vec<AllowedIp>> {
        use crate::schema::allowed_ips;
        use crate::schema::allowed_ips_clients::{self, dsl::*};
        allowed_ips_clients::table
            .inner_join(allowed_ips::table)
            .filter(client_id.eq(client.id))
            .select(allowed_ips::all_columns)
            .load::<AllowedIp>(conn)
            .map_err(|e| anyhow::Error::from(e).context("Could not get allowed IPs for client"))
    }

    /// This function assigns an allowed_ip to a client's pool of allowed source addresses, which
    /// will be used to build configuration on the "server" side
    pub fn assign_ip_to_client_sending(
        client: &Client,
        ip_address: &str,
        conn: &mut PgConnection,
    ) -> anyhow::Result<()> {
        use crate::schema::allowed_ips::dsl::*;
        use crate::schema::allowed_ips_clients_sending::dsl::*;

        let address_id = match AllowedIp::address_exists(ip_address, conn) {
            Ok(i) => i,
            Err(_) => diesel::insert_into(allowed_ips)
                .values(adress.eq(ip_address))
                .get_result::<AllowedIp>(conn)
                .map(|ip| ip.id)
                .map_err(|e| anyhow::Error::from(e).context("Could not insert allowed source IP"))?,
        };

        diesel::insert_into(allowed_ips_clients_sending)
            .values((client_id.eq(client.id), allowed_ip_ip.eq(address_id)))
            .execute(conn)
            .map(|_| ())
            .map_err(|e| anyhow::Error::from(e).context("Could not insert allowed source IP"))
    }

    /// This function retrieves allowed source addresses for a give client
    pub fn get_allowed_source_ips_for_client(
        client: &Client,
        conn: &mut PgConnection,
    ) -> anyhow::Result<Vec<AllowedIp>> {
        use crate::schema::allowed_ips;
        // allowed_ips_clients_sending
        use crate::schema::allowed_ips_clients_sending::{self, dsl::*};
        allowed_ips_clients_sending::table
            .inner_join(allowed_ips::table)
            .filter(client_id.eq(client.id))
            .select(allowed_ips::all_columns)
            .load::<AllowedIp>(conn)
            .map_err(|e| anyhow::Error::from(e).context("Could not get allowed source IPs for client"))
    }
}
