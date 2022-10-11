use async_graphql::{InputObject, SimpleObject};

use libwgbuilder::models::VpnNetwork as DbVpnNetwork;

#[derive(SimpleObject)]
pub struct VpnNetwork {
    id: i32,
    network: String,
    subnetmask: i32,
    interface: i32,
    port: i32,
}

impl From<DbVpnNetwork> for VpnNetwork {
    fn from(v: DbVpnNetwork) -> Self {
        VpnNetwork {
            id: v.id,
            network: v.network,
            subnetmask: v.subnetmask,
            interface: v.interface,
            port: v.port,
        }
    }
}

#[derive(InputObject)]
pub struct NewVpnNetwork {
    pub network: String,
    pub subnetmask: i32,
    pub interface: i32,
    pub port: i32,
}
