use anyhow::{anyhow, Error, Result};
use clap::{arg, command, Parser, Subcommand};
use graphql_client::{GraphQLQuery, Response};
use md5::{Digest, Md5};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

mod server_configuration;
use server_configuration::{
    server_configuration::{ResponseData as ServerResponse, Variables as ServerVariables},
    ServerConfiguration,
};
mod client_configuration;
use client_configuration::{
    client_configuration::{ResponseData as ClientResponse, Variables as ClientVariables},
    ClientConfiguration,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Path to the configuration file
    #[arg(short, long, default_value = "/etc/wireguard_client.conf")]
    pub configuration_path: String,
    /// Whether the wireguard client should be restarted after a configuration change
    #[arg(short, long)]
    restart_wireguard: bool,

    #[command(subcommand)]
    pub r#type: Type,
}

#[derive(Debug, Subcommand)]
enum Type {
    /// Get configuration for a server
    Server,
    /// Get configuration for a client
    Client,
}

#[derive(Serialize, Deserialize)]
struct Configuration {
    api_key: String,
    address: String,
    id: i32,
    interface_name: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            api_key: Default::default(),
            address: "http://localhost:8000".to_owned(),
            id: Default::default(),
            interface_name: "wg0".to_owned(),
        }
    }
}

/// Basic request builder
fn get_request(cfg: &Configuration) -> RequestBuilder {
    Client::new()
        .post(cfg.address.as_str())
        .header("Authorization", &format!("Token {}", &cfg.api_key))
}

/// Requests the configuration of the server and returns it
async fn server_configuration_query(cfg: &Configuration) -> Result<String> {
    let variables = ServerVariables {
        server_id: cfg.id as i64,
    };
    let request_body = ServerConfiguration::build_query(variables);
    let resp = get_request(cfg)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| Error::from(e).context("Could not send request to API"))?;
    let data: Response<ServerResponse> = resp
        .json()
        .await
        .map_err(|e| Error::from(e).context("Could not parse response from API"))?;
    match data.data {
        Some(d) => Ok(d.server.configuration),
        None => Err(anyhow!("No data in response. Token may be invalid")),
    }
}

/// Requests the configuration of the client and returns it
async fn client_configuration_query(cfg: &Configuration) -> Result<String> {
    let variables = ClientVariables {
        client_id: cfg.id as i64,
    };
    let request_body = ClientConfiguration::build_query(variables);
    let resp = get_request(cfg)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| Error::from(e).context("Could not send request to API"))?;
    let data: Response<ClientResponse> = resp
        .json()
        .await
        .map_err(|e| Error::from(e).context("Could not parse response from API"))?;
    match data.data {
        Some(d) => Ok(d.client.configuration),
        None => Err(anyhow!("No data in response. Token may be invalid")),
    }
}

/// Writes the configuration to the file and restarts the wireguard client if necessary
async fn get_and_write_wg_config(args: &Arguments, cfg: &Configuration) -> Result<()> {
    let configuration = match args.r#type {
        Type::Client => client_configuration_query(cfg).await,
        Type::Server => server_configuration_query(cfg).await,
    }?;
    let configuration_file = format!("/etc/wireguard/{}.conf", cfg.interface_name);

    let mut hasher = Md5::new();
    hasher.update(configuration.as_bytes());
    let new_conf_sum = hasher.finalize();

    let mut hasher = Md5::new();
    let old_conf = std::fs::read_to_string(&configuration_file);
    if let Err(e) = old_conf {
        if e.kind() != std::io::ErrorKind::NotFound {
            return Err(Error::from(e).context("Could not read old configuration file"));
        }
    } else {
        hasher.update(old_conf.unwrap().as_bytes());
    }

    let old_conf_sum = hasher.finalize();
    if new_conf_sum != old_conf_sum {
        std::fs::write(&configuration_file, configuration)?;
        if args.restart_wireguard {
            std::process::Command::new("wg-quick")
                .arg("down")
                .arg(&cfg.interface_name)
                .output()?;
            std::process::Command::new("wg-quick")
                .arg("up")
                .arg(&cfg.interface_name)
                .output()?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let cfg = confy::load_path::<Configuration>(&args.configuration_path)
        .expect("Could not read configuration file");

    if let Err(e) = get_and_write_wg_config(&args, &cfg).await {
        eprintln!("{}", e);
    } else {
        println!("Updated Config.");
    }
}
