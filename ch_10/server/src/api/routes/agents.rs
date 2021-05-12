use crate::api;
use common::api::RegisterAgent;
use std::{sync::Arc, time::Duration};
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

pub async fn get_agent_job(state: Arc<api::AppState>) -> Result<impl warp::Reply, warp::Rejection> {
    let sleep_for = Duration::from_secs(1);

    // long polling: 5 secs
    for _ in 0..5u64 {
        match state.service.get_agent_job().await? {
            Some(job) => {
                let res = api::Response::ok(job);
                let res_json = warp::reply::json(&res);
                return Ok(warp::reply::with_status(res_json, StatusCode::OK));
            }
            None => tokio::time::sleep(sleep_for).await,
        }
    }

    // if no job is found, return empty response
    let res = api::Response::<Option<()>>::ok(None);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}
