use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal")]
    Internal(String),
    #[error("Spider is not valid: {0}")]
    InvalidSpider(String),
}
