use crate::api;
use std::sync::Arc;
use warp::{http::StatusCode, Rejection};

pub async fn commands(_state: Arc<api::AppState>) -> Result<impl warp::Reply, Rejection> {
    let res = api::Response::ok(true);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}
