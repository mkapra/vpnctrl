#![allow(clippy::all, warnings)]
pub struct ServerConfiguration;
pub mod server_configuration {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ServerConfiguration";
    pub const QUERY : & str = "query ServerConfiguration($serverId: Int!) {\n  server(id: $serverId) {\n    configuration\n  }\n}" ;
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
        #[serde(rename = "serverId")]
        pub server_id: Int,
    }
    impl Variables {}
    #[derive(Debug, Deserialize)]
    pub struct ResponseData {
        pub server: ServerConfigurationServer,
    }
    #[derive(Debug, Deserialize)]
    pub struct ServerConfigurationServer {
        pub configuration: String,
    }
}
impl graphql_client::GraphQLQuery for ServerConfiguration {
    type Variables = server_configuration::Variables;
    type ResponseData = server_configuration::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: server_configuration::QUERY,
            operation_name: server_configuration::OPERATION_NAME,
        }
    }
}
