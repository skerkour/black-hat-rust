use std::fmt;

#[derive(Debug)]
pub enum Error {
    Internal(String),
    Trasnport(ureq::Error),
    Api(String),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for Error {}

impl std::convert::From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        match err {
            err @ ureq::Error::Transport(_) => Error::Trasnport(err),
            err @ ureq::Error::Status(_, _) => Error::Api(err.to_string()),
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::convert::From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<ed25519_dalek::SignatureError> for Error {
    fn from(err: ed25519_dalek::SignatureError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<chacha20poly1305::aead::Error> for Error {
    fn from(err: chacha20poly1305::aead::Error) -> Self {
        Error::Internal(err.to_string())
    }
}
