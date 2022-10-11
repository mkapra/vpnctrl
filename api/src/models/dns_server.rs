use async_graphql::{InputObject, SimpleObject};
use libwgbuilder::models::DnsServer as DbDnsServer;

#[derive(SimpleObject)]
pub struct DnsServer {
    id: i32,
    name: String,
    ip: String,
    description: Option<String>,
}

impl From<DbDnsServer> for DnsServer {
    fn from(s: DbDnsServer) -> Self {
        DnsServer {
            id: s.id,
            name: s.name,
            ip: s.ip,
            description: s.description,
        }
    }
}

#[derive(InputObject)]
pub struct NewDnsServer {
    pub name: String,
    pub ip: String,
    pub description: Option<String>,
}
