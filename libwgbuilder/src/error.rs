use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("Error while communicating with the database")]
    Database,
    #[error("The {0} with id `{1}` was not found")]
    DatabaseObjectNotFound(&'a str, i32),
    #[error("Unauthenticated")]
    Unauthenticated,
}
