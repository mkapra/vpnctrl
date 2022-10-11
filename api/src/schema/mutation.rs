use anyhow::Result;
use async_graphql::{Context, Object};

use libwgbuilder::models::keypair::NewKeypair as NewDbKeypair;

use super::get_db_connection;
use crate::models::Keypair;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn generate_keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        Ok(Keypair::from(NewDbKeypair::generate(&mut db)?))
    }
}
