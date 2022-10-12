#![allow(clippy::all, warnings)]
pub struct ClientConfiguration;
pub mod client_configuration {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ClientConfiguration";
    pub const QUERY : & str = "query ClientConfiguration($clientId: Int!) {\n  client(id: $clientId) {\n    configuration\n  }\n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "clientId")]
        pub client_id: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub client: ClientConfigurationClient,
    }
    #[derive(Deserialize)]
    pub struct ClientConfigurationClient {
        pub configuration: String,
    }
}
impl graphql_client::GraphQLQuery for ClientConfiguration {
    type Variables = client_configuration::Variables;
    type ResponseData = client_configuration::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: client_configuration::QUERY,
            operation_name: client_configuration::OPERATION_NAME,
        }
    }
}
