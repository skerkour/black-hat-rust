use crate::api;
use std::{convert::Infallible, sync::Arc};
use warp::http::StatusCode;

pub async fn commands(_state: Arc<crate::AppState>) -> Result<impl warp::Reply, Infallible> {
    let res = api::Response::ok(true);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}
