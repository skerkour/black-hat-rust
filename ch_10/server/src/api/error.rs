use super::Response;
use actix_web::{http::StatusCode, web::HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const EXTENSION_KEY_CODE: &str = "code";
const CODE_NOT_FOUND: &str = "NOT_FOUND";
const CODE_INTERNAL: &str = "INTERNAL";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Error {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, String>>,
}

impl std::convert::From<crate::Error> for Error {
    fn from(err: crate::Error) -> Self {
        match err {
            crate::Error::NotFound(err) => {
                let mut extensions = HashMap::new();
                extensions.insert(EXTENSION_KEY_CODE.into(), CODE_NOT_FOUND.into());

                Error {
                    message: err.to_string(),
                    extensions: Some(extensions),
                }
            }
            crate::Error::Internal(_) => {
                let mut extensions = HashMap::new();
                extensions.insert(EXTENSION_KEY_CODE.into(), CODE_INTERNAL.into());

                Error {
                    message: err.to_string(),
                    extensions: Some(extensions),
                }
            }
            _ => Error {
                message: err.to_string(),
                extensions: None,
            },
        }
    }
}

impl ResponseError for crate::Error {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let res = Response::<()>::err(self.clone());
        HttpResponse::build(self.status_code()).json(res)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            crate::Error::InvalidArgument(_) => StatusCode::BAD_REQUEST, // 400
            // Error::AuthenticationRequired => StatusCode::UNAUTHORIZED, // 401
            // Error::PermissionDenied(_) => StatusCode::FORBIDDEN,       // 403
            crate::Error::NotFound(_) => StatusCode::NOT_FOUND, // 404
            // Error::AlreadyExists(_) => StatusCode::CONFLICT,           // 409
            crate::Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500
        }
    }
}
