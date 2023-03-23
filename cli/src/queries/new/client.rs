use std::fmt::Display;

use anyhow::Result;
use graphql_client::{reqwest::post_graphql_blocking, GraphQLQuery};

use crate::queries::new::keypair::GenerateKeypair;

pub use self::new_client_information::{
    NewClientInformationDnsServers, NewClientInformationVpnNetworks,
};
use super::{build_client, State};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "graphql_queries/new_client.graphql",
    response_derives = "Debug"
)]
pub struct NewClient;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "graphql_queries/information_new_client.graphql",
    response_derives = "Debug"
)]
pub struct NewClientInformation;

impl NewClientInformation {
    pub fn create_client(
        ctx: &State,
        client_name: String,
        keepalive: i64,
        dns_server_id: Option<i64>,
        vpn_network_id: i64,
        ip_address: String,
    ) -> Result<()> {
        let keypair_id = GenerateKeypair::generate(ctx)?;

        let vpn_client = new_client::NewClient {
            name: client_name,
            description: None,
            dns_server_id,
            keepalive,
            keypair_id,
            vpn_ip: new_client::NewVpnIp {
                address: ip_address,
                vpn_network_id,
            },
        };
        let client = build_client(ctx)?;
        let variables = new_client::Variables {
            new_client: vpn_client,
        };
        let res = post_graphql_blocking::<NewClient, _>(&client, &ctx.url, variables)?;
        println!(
            "Client with id {} successfully created",
            res.data.expect("No response data found").new_client.id
        );
        Ok(())
    }

    /// Returns informations necessary for user inputs
    pub fn get(ctx: &State) -> Result<new_client_information::ResponseData> {
        let client = build_client(ctx)?;
        let res = post_graphql_blocking::<Self, _>(
            &client,
            &ctx.url,
            new_client_information::Variables {},
        )
        .unwrap();
        Ok(res.data.expect("Missing response data"))
    }

    /// Returns a DNS server dummy that represents the option, that no DNS server should be used
    pub fn no_dns() -> NewClientInformationDnsServers {
        NewClientInformationDnsServers::no_dns()
    }
}

impl Display for NewClientInformationDnsServers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.id == -1 {
            return write!(f, "No DNS Server");
        }
        match self.description.clone() {
            Some(d) => write!(f, "{}: {} ({})", self.name, d, self.ip),
            None => write!(f, "{} ({})", self.name, self.ip),
        }
    }
}

impl NewClientInformationDnsServers {
    fn no_dns() -> Self {
        Self {
            id: -1,
            name: "".to_string(),
            ip: "".to_string(),
            description: None,
        }
    }
}

impl Display for NewClientInformationVpnNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.network, self.subnetmask)
    }
}
