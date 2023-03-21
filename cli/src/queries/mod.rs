use std::fmt::Display;

use anyhow::Error;
use anyhow::Result;
use graphql_client::{reqwest::post_graphql_blocking, GraphQLQuery};
use inquire::{required, Select, Text};
use reqwest::blocking::Client;

use self::new_client_information::NewClientInformationDnsServers;
use self::new_client_information::NewClientInformationVpnNetworks;
use crate::State;

fn build_client(ctx: &State) -> Result<Client> {
    Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", ctx.jwt_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()
        .map_err(|e| Error::from(e))
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/login.graphql",
    response_derives = "Debug"
)]
pub struct Login;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/generate_keypair.graphql",
    response_derives = "Debug"
)]
struct GenerateKeypair;

impl GenerateKeypair {
    fn generate(ctx: &State) -> Result<i64> {
        let client = build_client(ctx)?;
        let res = post_graphql_blocking::<GenerateKeypair, _>(
            &client,
            &ctx.url,
            generate_keypair::Variables {},
        )
        .unwrap();
        let res_data = res.data.expect("Missing response data");
        Ok(res_data.generate_keypair.id)
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/new_client.graphql",
    response_derives = "Debug"
)]
pub struct NewClient;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/information_new_client.graphql",
    response_derives = "Debug"
)]
pub struct NewClientInformation;

impl Display for NewClientInformationDnsServers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.description.clone() {
            Some(d) => write!(f, "{}: {} ({})", self.name, d, self.ip),
            None => write!(f, "{} ({})", self.name, self.ip),
        }
    }
}

impl Display for NewClientInformationVpnNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.network, self.subnetmask)
    }
}

impl NewClientInformation {
    pub fn create_client(ctx: &State) -> Result<()> {
        let keypair_id = GenerateKeypair::generate(ctx)?;
        let information = Self::get(ctx)?;

        let client_name = Text::new("Name of the client:")
            .with_validator(required!())
            .prompt()?;
        let keepalive = Text::new("Keepalive:").with_default("25").prompt()?;
        let dns_server_id = Select::new("DNS server to use", information.dns_servers)
            .prompt()?
            .id;
        let vpn_network_id = Select::new("VPN Network", information.vpn_networks)
            .prompt()?
            .id;
        let ip_address = Text::new("IP address in VPN network:").prompt()?;

        let vpn_client = new_client::NewClient {
            name: client_name,
            description: None,
            dns_server_id: Some(dns_server_id),
            keepalive: keepalive.parse::<i64>().expect("Keepalive is not a number"),
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

    fn get(ctx: &State) -> Result<new_client_information::ResponseData> {
        let client = build_client(ctx)?;
        let res = post_graphql_blocking::<Self, _>(
            &client,
            &ctx.url,
            new_client_information::Variables {},
        )
        .unwrap();
        Ok(res.data.expect("Missing response data"))
    }
}
