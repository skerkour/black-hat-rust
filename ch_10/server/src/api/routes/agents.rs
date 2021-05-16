use crate::api::AppState;
use common::api;
use std::sync::Arc;
use warp::{http::StatusCode, Rejection};

pub async fn get_agents(state: Arc<AppState>) -> Result<impl warp::Reply, Rejection> {
    let agents = state.service.list_agents().await?;
    let agents = agents.into_iter().map(Into::into).collect();
    let res = api::AgentsList { agents };

    let res = api::Response::ok(res);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}

pub async fn post_agents(state: Arc<AppState>) -> Result<impl warp::Reply, Rejection> {
    let agent_info = state.service.register_agent().await?;

    let res = api::Response::ok(agent_info);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}
