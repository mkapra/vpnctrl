use async_graphql::http::graphiql_source;
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use database::DatabaseConn;
use rocket::{get, launch, post, response::content, routes, State};

mod schema;
use schema::{build_schema, WireguardSchema};
mod database;
mod models;

#[get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(graphiql_source("/graphql", None))
}

#[get("/graphql?<query..>")]
async fn graphql_query(schema: &State<WireguardSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: &State<WireguardSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema).await
}

#[launch]
fn launch() -> _ {
    let url = "postgres://localhost/wgbuilder";
    let pool = DatabaseConn::new(url).expect("Could not build database connection pool");
    let schema = build_schema(pool);

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_query, graphql_request, graphiql])
}
