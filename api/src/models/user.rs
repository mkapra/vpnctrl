use async_graphql::InputObject;

#[derive(InputObject)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub role: i32,
}
