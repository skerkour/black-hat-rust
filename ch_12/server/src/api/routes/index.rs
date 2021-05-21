use common::api;
use std::collections::HashMap;
use warp::{http::StatusCode, Rejection};

pub async fn index() -> Result<impl warp::Reply, Rejection> {
    let mut data = HashMap::new();
    data.insert("hello", "world");

    let res = api::Response::ok(data);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}
