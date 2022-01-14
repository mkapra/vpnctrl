//! Module that holds everything that is necessary for the `VpnNetwork`
use async_graphql::*;

use super::vpn_ip_address::VpnIpAddress;
use super::*;
use crate::schema::{vpn_ip_addresses, vpn_networks};

/// A [`VpnNetwork`] that is insertable into the database
#[derive(Insertable)]
#[table_name = "vpn_networks"]
pub struct NewVpnNetwork<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub ip_network: &'a str,
    pub subnetmask: i32,
    pub listen_port: i32,
    pub interface_name: &'a str,
}

/// Input type for creating a new VpnNetwork
#[derive(InputObject)]
pub struct InputVpnNetwork {
    pub name: String,
    pub description: Option<String>,
    /// The ip_network that the object represents
    #[graphql(validator(ip))]
    pub ip_network: String,
    /// The subnetmask of the ip_network in CIDR format
    #[graphql(default = 24)]
    pub subnetmask: i32,
    /// The port where the vpn server is listening on
    pub listen_port: i32,
    /// The name of the interface (e.g. wg0)
    pub interface_name: String,
}

/// A VpnNetwork represents a network that contains clients and a server
#[derive(Debug, SimpleObject, Queryable, Identifiable, AsChangeset)]
#[graphql(complex)]
pub struct VpnNetwork {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    /// The ip_network that the object represents
    pub ip_network: String,
    /// The subnetmask of the ip_network in CIDR format
    pub subnetmask: i32,
    /// The port where the vpn server is listening on
    pub listen_port: i32,
    /// The name of the interface (e.g. wg0)
    pub interface_name: String,
}

#[ComplexObject]
impl VpnNetwork {
    /// All Clients that are associated with the VpnNetwork
    async fn clients(&self, ctx: &Context<'_>) -> Result<Vec<Client>> {
        match self.get_associated_clients(&create_connection(ctx)) {
            Some(clients) => {
                return Ok(clients.into_iter().map(Client::from).collect());
            }
            None => return Ok(vec![]),
        }
    }

    /// The Server that is associated with the VpnNetwork
    async fn server(&self, ctx: &Context<'_>) -> Option<Server> {
        self.get_associated_server(&create_connection(ctx))
            .map(|qserver| Server::from(qserver))
    }
}

impl VpnNetwork {
    /// Creates a new [`VpnNetwork`] in the database
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `vpn_network` - The [`VpnNetwork`] that should be created
    ///
    /// # Returns
    /// Returns the created [`VpnNetwork`] or an error that was thrown by the database
    pub fn create(
        connection: &DatabaseConnection,
        vpn_network: &InputVpnNetwork,
    ) -> Result<VpnNetwork> {
        let new_vpn_network = NewVpnNetwork {
            name: &vpn_network.name,
            description: vpn_network.description.as_deref(),
            ip_network: &vpn_network.ip_network,
            subnetmask: vpn_network.subnetmask,
            listen_port: vpn_network.listen_port,
            interface_name: &vpn_network.interface_name,
        };

        diesel::insert_into(vpn_networks::table)
            .values(&new_vpn_network)
            .get_result(connection)
            .map_err(Error::from)
    }

    /// Updates a [`VpnNetwork`] in the database
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `net_id` - The id of the [`VpnNetwork`] that should be updated
    /// * `vpn_network` - The new [`VpnNetwork`] object that should replace the old one
    ///
    /// # Returns
    /// The update may return an error if the new values violate unique constraints in the database. Otherwise the
    /// updated vpn network is returned.
    pub fn update(
        connection: &DatabaseConnection,
        net_id: i32,
        vpn_network: &InputVpnNetwork,
    ) -> Result<VpnNetwork> {
        if let Some(net) = Self::get_by_id(connection, net_id) {
            let updated_net = VpnNetwork {
                id: net.id,
                name: vpn_network.name.clone(),
                description: vpn_network.description.clone(),
                ip_network: vpn_network.ip_network.clone(),
                subnetmask: vpn_network.subnetmask,
                listen_port: vpn_network.listen_port,
                interface_name: vpn_network.interface_name.clone(),
            };
            return diesel::update(&net)
                .set(&updated_net)
                .get_result(connection)
                .map_err(Error::from);
        }

        Err(Error::new(format!(
            "VPN Network with id {} not found",
            net_id
        )))
    }

    /// Deletes a [`VpnNetwork`] from the database
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `net_id` - The id of the [`VpnNetwork`] that should be deleted
    ///
    /// # Returns
    /// Returns true if the delete action was successful an error otherwise
    pub fn delete(connection: &DatabaseConnection, net_id: i32) -> Result<bool> {
        match Self::get_by_id(connection, net_id) {
            Some(net) => match diesel::delete(&net).execute(connection) {
                Ok(_) => Ok(true),
                Err(e) => Err(Error::from(e)),
            },
            None => Err(Error::new(format!(
                "VPN Network with id {} not found",
                net_id
            ))),
        }
    }

    /// Returns the [`VpnNetwork`] for the given id
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `net_id` - The id of the vpn network that should be returned
    ///
    /// # Returns
    /// If a vpn network was found a [`Option::Some`] will be returned [`Option::None`] otherwise
    pub fn get_by_id(connection: &DatabaseConnection, net_id: i32) -> Option<VpnNetwork> {
        use crate::schema::vpn_networks::dsl::*;

        vpn_networks
            .filter(id.eq(net_id))
            .load::<VpnNetwork>(connection)
            .expect("Could not query the database")
            .pop()
    }

    /// Returns the `Client`s that are associated with the [`VpnNetwork`]
    pub fn get_associated_clients(
        &self,
        connection: &DatabaseConnection,
    ) -> Option<Vec<QueryableClient>> {
        use crate::schema::clients::dsl::*;

        match clients
            .filter(vpn_ip_addresses::vpn_network_id.eq(self.id))
            .inner_join(vpn_ip_addresses::table)
            .load(connection)
        {
            Ok(results) => {
                let mapped_clients = results
                    .into_iter()
                    .map(|(c, _): (QueryableClient, VpnIpAddress)| c)
                    .collect();

                return Some(mapped_clients);
            }
            Err(_) => return None,
        }
    }

    /// Returns the `Server` that is associated with the [`VpnNetwork`]
    pub fn get_associated_server(
        &self,
        connection: &DatabaseConnection,
    ) -> Option<QueryableServer> {
        use crate::schema::servers::dsl::*;

        match servers
            .filter(vpn_ip_addresses::vpn_network_id.eq(self.id))
            .inner_join(vpn_ip_addresses::table)
            .first::<(QueryableServer, VpnIpAddress)>(connection)
        {
            Ok((server, _)) => {
                return Some(server);
            }
            Err(_) => return None,
        }
    }
}
