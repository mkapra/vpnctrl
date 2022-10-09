use diesel::PgConnection;

pub mod keypair;
pub use keypair::Keypair;

pub mod dns_server;
pub use dns_server::DnsServer;

pub mod vpn_network;
pub use vpn_network::VpnNetwork;

pub mod vpn_ip;
pub use vpn_ip::VpnIp;

pub mod client;
pub use client::Client;

pub mod server;
pub use server::Server;

pub trait Model {
    /// Finds the object for the given `search_id` in the database
    ///
    /// # Returns
    /// If the object was not found an `Err` will be returned.
    fn find(search_id: i32, conn: &mut PgConnection) -> anyhow::Result<Self>
    where
        Self: Sized;

    /// Deletes the given object from the database
    fn delete(self, conn: &mut PgConnection) -> anyhow::Result<()>;
}
