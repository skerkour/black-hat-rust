use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
}

impl std::convert::From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Internal(err.to_string())
    }
}
