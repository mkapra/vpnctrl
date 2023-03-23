use anyhow::{anyhow, Error, Result};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};

use super::{build_client, State};

/// Login query which retrieves a JWT token necessary for all the other actions
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "graphql_queries/login.graphql",
    response_derives = "Debug"
)]
pub struct Login;

impl Login {
    pub fn query(ctx: &State, username: String, password: String) -> Result<String> {
        let client = build_client(ctx)?;
        let variables = login::Variables { username, password };

        let res = post_graphql::<Login, _>(&client, &ctx.url, variables).map_err(|e| {
            Error::from(e).context(format!(
                "Could not login to {}. Wrong username or password?",
                ctx.url
            ))
        })?;
        res.data
            .map(|d| d.login)
            .ok_or(anyhow!("Could not get login response from API"))
    }
}
