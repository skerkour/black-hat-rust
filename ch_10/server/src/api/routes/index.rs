use crate::api;
use std::{collections::HashMap, convert::Infallible};
use warp::http::StatusCode;

pub async fn index() -> Result<impl warp::Reply, Infallible> {
    let mut data = HashMap::new();
    data.insert("hello", "world");
    let res = api::Response::ok(data);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}
