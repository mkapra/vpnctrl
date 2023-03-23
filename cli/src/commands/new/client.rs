//! Create a new client
use anyhow::{Error, Result};
use inquire::{required, Select, Text};

use crate::{queries::NewClientInformation, State};

pub fn new_client(_args: Vec<String>, ctx: &mut State) -> Result<()> {
    let information = NewClientInformation::get(&ctx)
        .map_err(|e| e.context("Did not receive any informations"))?;

    let client_name = Text::new("Name of the client:")
        .with_validator(required!())
        .prompt()
        .map_err(|e| Error::from(e).context("Could not get client name from user"))?;
    let keepalive = Text::new("Keepalive:")
        .with_default("25")
        .prompt()
        .map_err(|e| Error::from(e).context("Could not get keepalive from user"))?
        .parse::<i64>()
        .map_err(|e| Error::from(e).context("Given keepalive is not a number"))?;

    let mut dns_servers_from_api = information.dns_servers;
    let mut dns_servers = vec![NewClientInformation::no_dns()];
    dns_servers.append(&mut dns_servers_from_api);
    let dns_server_id = Select::new("DNS server to use", dns_servers)
        .prompt()
        .map_err(|e| Error::from(e).context("Could not get DNS server selection from user"))?
        .id;
    let dns_server_id = match dns_server_id {
        -1 => None,
        x => Some(x),
    };

    let vpn_network_id = Select::new("VPN Network", information.vpn_networks)
        .prompt()
        .map_err(|e| Error::from(e).context("Could not get VPN network selection from user"))?
        .id;
    let ip_address = Text::new("IP address in VPN network:")
        .prompt()
        .map_err(|e| Error::from(e).context("Could not get ip address from user"))?;

    NewClientInformation::create_client(
        &ctx,
        client_name,
        keepalive,
        dns_server_id,
        vpn_network_id,
        ip_address,
    )
}
