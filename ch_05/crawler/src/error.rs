use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal")]
    Internal(String),
    #[error("Spider is not valid: {0}")]
    InvalidSpider(String),
    #[error("Reqwest: {0}")]
    Reqwest(String),
    #[error("WebDriver: {0}")]
    WebDriver(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

impl std::convert::From<fantoccini::error::CmdError> for Error {
    fn from(err: fantoccini::error::CmdError) -> Self {
        Error::WebDriver(err.to_string())
    }
}

impl std::convert::From<fantoccini::error::NewSessionError> for Error {
    fn from(err: fantoccini::error::NewSessionError) -> Self {
        Error::WebDriver(err.to_string())
    }
}
