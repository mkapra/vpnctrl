use diesel::PgConnection;

pub mod keypair;
pub use keypair::Keypair;

pub mod dns_server;
pub use dns_server::DnsServer;

pub mod vpn_network;
pub use vpn_network::VpnNetwork;

pub trait Model {
    /// Finds the object for the given `search_id` in the database
    ///
    /// # Returns
    /// If the object was not found an `Err` will be returned.
    fn find(search_id: i32, conn: &mut PgConnection) -> anyhow::Result<Self>
    where
        Self: Sized;
}
