use std::env;

use async_graphql::{http::GraphiQLSource, Request, Response};
use database::DatabaseConn;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use poem::{
    get, handler,
    http::{Method, StatusCode},
    listener::TcpListener,
    middleware::Cors,
    web::{Data, Html, Json},
    EndpointExt, FromRequest, IntoResponse, RequestBody, Route, Server,
};

mod schema;
use schema::{build_schema, WireguardSchema};

mod auth;
mod database;
mod models;
use auth::jwt::Secret;

use auth::ApiKey;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../libwgbuilder/migrations/");

struct CheckForDebug;

#[poem::async_trait]
impl<'a> FromRequest<'a> for CheckForDebug {
    async fn from_request(_req: &'a poem::Request, _body: &mut RequestBody) -> poem::Result<Self> {
        return match cfg!(debug_assertions) {
            true => Ok(CheckForDebug),
            false => Err(poem::Error::from_string("", StatusCode::NOT_FOUND)),
        };
    }
}

#[handler]
async fn graphiql(_: CheckForDebug) -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[handler]
async fn graphql_handler(
    schema: Data<&WireguardSchema>,
    req: Json<Request>,
    api_key: ApiKey,
) -> Json<Response> {
    Json(schema.execute(req.0.data(api_key)).await)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listen_ip = env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let url = env::var("DATABASE_URL").expect("Could not find DATABASE_URL");
    let secret = env::var("SECRET").expect("Could not find SECRET for JWT tokens");
    let pool = DatabaseConn::new(&url).expect("Could not build database connection pool");
    let mut db = pool.get().expect("Could not get database connection");
    let schema = build_schema(pool, Secret::new(&secret));
    db.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");

    let app = Route::new()
        .at("/", get(graphiql).post(graphql_handler))
        .with(
            Cors::new()
                .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
                .allow_credentials(true),
        )
        .data(schema);

    Server::new(TcpListener::bind(format!("{}:3000", listen_ip)))
        .run(app)
        .await
}
