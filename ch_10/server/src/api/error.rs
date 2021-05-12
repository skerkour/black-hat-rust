// use super::Response;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

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

pub async fn handle_error(rejection: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let status;
    let err;

    if rejection.is_not_found() {
        status = StatusCode::NOT_FOUND;
        err = crate::Error::NotFound("Route not found.".to_string());
    } else if let Some(_) = rejection.find::<warp::filters::body::BodyDeserializeError>() {
        status = StatusCode::BAD_REQUEST;
        err = crate::Error::InvalidArgument("Invalid Body.".to_string());
    } else if let Some(_) = rejection.find::<warp::reject::MethodNotAllowed>() {
        status = StatusCode::METHOD_NOT_ALLOWED;
        err = crate::Error::InvalidArgument("Invalid HTTP Method.".to_string());
    } else if let Some(e) = rejection.find::<crate::Error>() {
        status = match e {
            crate::Error::InvalidArgument(_) => StatusCode::BAD_REQUEST, // 400
            // Error::AuthenticationRequired => StatusCode::UNAUTHORIZED, // 401
            // Error::PermissionDenied(_) => StatusCode::FORBIDDEN,       // 403
            crate::Error::NotFound(_) => StatusCode::NOT_FOUND, // 404
            // Error::AlreadyExists(_) => StatusCode::CONFLICT,           // 409
            crate::Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500
        };
        err = e.to_owned();
    } else {
        status = StatusCode::INTERNAL_SERVER_ERROR;
        err = crate::Error::Internal("".to_string());
    }

    let res = super::Response::<()>::err(err);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, status))
}
