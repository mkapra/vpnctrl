use async_graphql::SimpleObject;
use libwgbuilder::models::Keypair as DbKeypair;

#[derive(SimpleObject)]
pub struct Keypair {
    id: i32,
    public_key: String,
    private_key: String,
}

impl From<DbKeypair> for Keypair {
    fn from(keypair: DbKeypair) -> Self {
        Self {
            id: keypair.id,
            public_key: keypair.public_key,
            private_key: keypair.private_key,
        }
    }
}
