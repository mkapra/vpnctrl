use anyhow::{anyhow, bail, Result};
use async_graphql::{Context, Object};

use libwgbuilder::models::client::{Client as DbClient, NewClient as NewDbClient};
use libwgbuilder::models::dns_server::NewDnsServer as NewDbDnsServer;
use libwgbuilder::models::keypair::NewKeypair as NewDbKeypair;
use libwgbuilder::models::server::NewServer as NewDbServer;
use libwgbuilder::models::token::NewToken;
use libwgbuilder::models::user::NewUser as NewDbUser;
use libwgbuilder::models::vpn_ip::NewVpnIp;
use libwgbuilder::models::vpn_network::NewVpnNetwork as NewDbVpnNetwork;
use libwgbuilder::models::AllowedIp as DbAllowedIp;
use libwgbuilder::models::{Model, User};

use super::get_db_connection;
use crate::auth::jwt;
use crate::auth::{
    jwt::{encode_jwt, Claims},
    AdminGuard,
};
use crate::models::server::NewServer;
use crate::models::user::NewUser;
use crate::models::Server;
use crate::models::{
    client::NewClient, dns_server::NewDnsServer, vpn_network::NewVpnNetwork, Client, DnsServer,
    Keypair, VpnNetwork,
};

pub struct Mutation;

#[Object]
impl Mutation {
    /// Generates a keypair
    #[graphql(guard = "AdminGuard::new()")]
    async fn generate_keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        Ok(Keypair::from(NewDbKeypair::generate(&mut db)?))
    }

    /// Creates a DNS Server
    #[graphql(guard = "AdminGuard::new()")]
    async fn new_dns_server(
        &self,
        ctx: &Context<'_>,
        dns_server: NewDnsServer,
    ) -> Result<DnsServer> {
        let mut db = get_db_connection(ctx)?;
        Ok(DnsServer::from(
            NewDbDnsServer::new(&dns_server.name, &dns_server.ip, dns_server.description)
                .create(&mut db)?,
        ))
    }

    /// Creates a VPN network
    #[graphql(guard = "AdminGuard::new()")]
    async fn new_vpn_network(
        &self,
        ctx: &Context<'_>,
        vpn_network: NewVpnNetwork,
    ) -> Result<VpnNetwork> {
        let mut db = get_db_connection(ctx)?;
        Ok(VpnNetwork::from(
            NewDbVpnNetwork::new(
                &vpn_network.network,
                vpn_network.subnetmask,
                vpn_network.interface,
                vpn_network.port,
            )
            .create(&mut db)?,
        ))
    }

    /// Creates a client
    #[graphql(guard = "AdminGuard::new()")]
    async fn new_client(&self, ctx: &Context<'_>, client: NewClient) -> Result<Client> {
        let mut db = get_db_connection(ctx)?;

        // Create VPN IP address
        let ip =
            NewVpnIp::new(&client.vpn_ip.address, client.vpn_ip.vpn_network_id).create(&mut db)?;

        Ok(Client::from(
            NewDbClient::new(
                &client.name,
                client.description,
                client.dns_server_id,
                client.keepalive,
                client.keypair_id,
                ip.id,
            )
            .create(&mut db)?,
        ))
    }

    /// Creates a server
    #[graphql(guard = "AdminGuard::new()")]
    async fn new_server(&self, ctx: &Context<'_>, server: NewServer) -> Result<Server> {
        let mut db = get_db_connection(ctx)?;

        // Create VPN IP address
        let ip =
            NewVpnIp::new(&server.vpn_ip.address, server.vpn_ip.vpn_network_id).create(&mut db)?;

        Ok(Server::from(
            NewDbServer::new(
                &server.name,
                server.description,
                server.forward_interface,
                server.keypair_id,
                ip.id,
                &server.external_ip,
            )
            .create(&mut db)?,
        ))
    }

    /// Assign a token to the given server
    ///
    /// This token can be used to retrieve the configuration for this particular server
    #[graphql(guard = "AdminGuard::new()")]
    async fn generate_token_for_server(&self, ctx: &Context<'_>, server_id: i32) -> Result<String> {
        let mut db = get_db_connection(ctx)?;

        let token = NewToken::generate(&mut db)?;
        token.assign_to_server(server_id, &mut db)?;

        Ok(token.token)
    }

    /// Assign a token to the given client
    ///
    /// This token can be used to retrieve the configuration for this particular client
    #[graphql(guard = "AdminGuard::new()")]
    async fn generate_token_for_client(&self, ctx: &Context<'_>, client_id: i32) -> Result<String> {
        let mut db = get_db_connection(ctx)?;

        let token = NewToken::generate(&mut db)?;
        token.assign_to_client(client_id, &mut db)?;

        Ok(token.token)
    }

    /// Creates a new administrator account
    #[graphql(guard = "AdminGuard::new()")]
    async fn generate_administrator(&self, ctx: &Context<'_>, user: NewUser) -> Result<bool> {
        let mut db = get_db_connection(ctx)?;
        NewDbUser::new(&user.username, &user.password, 0)
            .create(&mut db)
            .map(|_| true)
    }

    /// Returns a token for an user
    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<String> {
        let mut db = get_db_connection(ctx)?;
        let secret = ctx
            .data::<jwt::Secret>()
            .map_err(|_| anyhow!("Could not get secret key"))?;
        let user = User::find_by_username(&username, &mut db)?;
        if !bcrypt::verify(password, &user.password)? {
            bail!("Not authenticated")
        }

        encode_jwt(&Claims::new(user.username, user.role), &secret.0)
    }

    /// Changes the password for the given username
    #[graphql(guard = "AdminGuard::new()")]
    async fn change_password(
        &self,
        ctx: &Context<'_>,
        username: String,
        old_password: String,
        new_password: String,
    ) -> Result<bool> {
        let mut db = get_db_connection(ctx)?;
        let user = User::find_by_username(&username, &mut db)?;

        if bcrypt::verify(&old_password, &user.password)? {
            user.update_password(&new_password, &mut db)?;
            return Ok(true);
        }

        bail!("Invalid password")
    }

    /// Assigns an allowed IP to a client
    #[graphql(guard = "AdminGuard::new()")]
    async fn assign_allowed_ip(
        &self,
        ctx: &Context<'_>,
        client_id: i32,
        allowed_ip: String,
    ) -> Result<bool> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(client_id, &mut db)?;
        DbAllowedIp::assign_ip_to_client(&client, &allowed_ip, &mut db)?;
        Ok(true)
    }

    /// Assigns an allowed source IP to a client
    #[graphql(guard = "AdminGuard::new()")]
    async fn assign_allowed_source_ip(
        &self,
        ctx: &Context<'_>,
        client_id: i32,
        allowed_ip: String,
    ) -> Result<bool> {
        let mut db = get_db_connection(ctx)?;
        let client = DbClient::find(client_id, &mut db)?;
        DbAllowedIp::assign_ip_to_client_sending(&client, &allowed_ip, &mut db)?;
        Ok(true)
    }
}
