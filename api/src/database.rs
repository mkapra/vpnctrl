//! Representation of the database connection layer
use anyhow::{Error, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

/// Pool for database connections
pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;
/// Single connection to the database
pub type DatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// This struct is responsible for the database connection pool
///
/// It hides the pool from the outside and provides necessary methods for getting a single database connection
pub struct DatabaseConn(DatabasePool);

impl DatabaseConn {
    /// Creates a new connection pool
    ///
    /// # Returns
    /// This function returns an `Err` if the database connection could not be established
    pub fn new(url: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .map(DatabaseConn)
            .map_err(|e| Error::from(e).context("Could not build database connection pool"))
    }

    /// Returns a single connection from the connection pool
    pub fn get(&self) -> Result<DatabaseConnection> {
        self.0
            .get()
            .map_err(|e| Error::from(e).context("Could not get connection from database pool"))
    }
}
