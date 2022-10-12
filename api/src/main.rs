use async_graphql::http::GraphiQLSource;
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use database::DatabaseConn;
use rocket::{get, launch, post, response::content, routes, State};

mod schema;
use schema::{build_schema, WireguardSchema};

mod auth;
mod database;
mod models;
use auth::{jwt::Secret, ApiKey};

#[get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: &State<WireguardSchema>,
    request: GraphQLRequest,
    api_key: ApiKey,
) -> GraphQLResponse {
    request.data(api_key).execute(schema).await
}

#[launch]
fn launch() -> _ {
    let url = "postgres://localhost/wgbuilder";
    let secret = "123123";
    let pool = DatabaseConn::new(url).expect("Could not build database connection pool");
    let schema = build_schema(pool, Secret::new(secret));

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_request, graphiql])
}
