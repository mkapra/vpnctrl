use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error while communicating with the database")]
    Database,
}
