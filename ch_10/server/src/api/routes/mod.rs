use crate::{api, Error};
use std::convert::Infallible;
use warp::http::StatusCode;

mod commands;
mod index;

pub use commands::commands;
pub use index::index;

pub async fn not_found() -> Result<impl warp::Reply, Infallible> {
    let err = Error::NotFound("Route not found".to_string());
    let res = api::Response::<()>::err(err);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::NOT_FOUND))
}
