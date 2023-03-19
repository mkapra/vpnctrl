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

pub mod token;
pub use token::Token;

pub mod user;
pub use user::User;

pub mod allowed_ip;
pub use allowed_ip::AllowedIp;

pub trait Model {
    /// Finds the object for the given `search_id` in the database
    ///
    /// # Returns
    /// If the object was not found an `Err` will be returned.
    fn find(search_id: i32, conn: &mut PgConnection) -> anyhow::Result<Self>
    where
        Self: Sized;

    /// Updates the given object in the database
    fn edit(&mut self, conn: &mut PgConnection) -> anyhow::Result<()>;
}
