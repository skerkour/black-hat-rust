use crate::api;
use common::api::RegisterAgent;
use std::sync::Arc;
use warp::{http::StatusCode, Rejection};

pub async fn get_agents(state: Arc<api::AppState>) -> Result<impl warp::Reply, Rejection> {
    let _ = state.service.list_agents().await?;

    let res = api::Response::ok(true);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}

pub async fn post_agents(
    state: Arc<api::AppState>,
    input: RegisterAgent,
) -> Result<impl warp::Reply, Rejection> {
    let _ = state.service.register_agent(input).await?;

    let res = api::Response::ok(true);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}
