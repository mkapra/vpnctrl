use anyhow::Result;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};

use super::{build_client, State};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "graphql_queries/generate_keypair.graphql",
    response_derives = "Debug"
)]
pub struct GenerateKeypair;

impl GenerateKeypair {
    pub fn generate(ctx: &State) -> Result<i64> {
        let client = build_client(ctx)?;
        let res =
            post_graphql::<GenerateKeypair, _>(&client, &ctx.url, generate_keypair::Variables {})
                .unwrap();
        let res_data = res.data.expect("Missing response data");
        Ok(res_data.generate_keypair.id)
    }
}
