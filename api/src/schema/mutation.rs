use async_graphql::{Object, Context};
use anyhow::Result;

use libwgbuilder::models::keypair::NewKeypair as NewDbKeypair;

use crate::models::Keypair;
use super::get_db_connection;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn generate_keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        let mut db = get_db_connection(ctx)?;
        Ok(Keypair::from(NewDbKeypair::generate(&mut db)?))
    }
}