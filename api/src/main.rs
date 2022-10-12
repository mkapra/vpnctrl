use std::env;

use async_graphql::http::GraphiQLSource;
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use database::DatabaseConn;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::{
    get,
    http::{Method, Status},
    launch, post,
    request::Outcome,
    request::{self, FromRequest},
    response::content,
    routes, Request, State,
};

mod schema;
use rocket_cors::AllowedHeaders;
use schema::{build_schema, WireguardSchema};

mod auth;
mod database;
mod models;
use auth::{jwt::Secret, ApiKey};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../libwgbuilder/migrations/");

pub struct SkipGraphiQL;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SkipGraphiQL {
    type Error = anyhow::Error;

    async fn from_request(_: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match cfg!(debug_assertions) {
            true => Outcome::Success(SkipGraphiQL {}),
            false => Outcome::Failure((Status::NotFound, anyhow::anyhow!("Not found"))),
        }
    }
}

#[get("/")]
fn graphiql(_skip_graphiql: SkipGraphiQL) -> content::RawHtml<String> {
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
    let url = env::var("DATABASE_URL").expect("Could not find DATABASE_URL");
    let secret = env::var("SECRET").expect("Could not find SECRET for JWT tokens");
    let pool = DatabaseConn::new(&url).expect("Could not build database connection pool");
    let mut db = pool.get().expect("Could not get database connection");
    let schema = build_schema(pool, Secret::new(&secret));
    db.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");

    // Init rocket_cors CORS
    let allowed_origins = rocket_cors::AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Could not build CORS");

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_request, graphiql])
        .attach(cors)
}
