use serde::de::DeserializeOwned;
use warp::Filter;

mod error;
mod response;

pub mod routes;
pub use error::Error;
pub use response::Response;

pub fn json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
