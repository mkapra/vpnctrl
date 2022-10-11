use anyhow::Result;
use async_graphql::ComplexObject;
use async_graphql::Context;
use async_graphql::SimpleObject;
use libwgbuilder::models::Model;
use libwgbuilder::models::VpnIp as DbVpnIp;
use libwgbuilder::models::VpnNetwork as DbVpnNetwork;

use crate::schema::get_db_connection;

use super::VpnNetwork;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct VpnIp {
    id: i32,
    address: String,
}

impl From<DbVpnIp> for VpnIp {
    fn from(i: DbVpnIp) -> Self {
        VpnIp {
            id: i.id,
            address: i.address,
        }
    }
}

#[ComplexObject]
impl VpnIp {
    async fn vpn_network(&self, ctx: &Context<'_>) -> Result<VpnNetwork> {
        let mut db = get_db_connection(ctx)?;
        let vpn_ip = DbVpnIp::find(self.id, &mut db)?;
        Ok(VpnNetwork::from(DbVpnNetwork::find(
            vpn_ip.vpn_network_id,
            &mut db,
        )?))
    }
}
