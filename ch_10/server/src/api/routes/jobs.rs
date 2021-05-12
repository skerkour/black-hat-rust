use crate::api;
use common::api::CreateJob;
use std::{sync::Arc, time::Duration};
use uuid::Uuid;
use warp::http::StatusCode;

pub async fn create_job(
    state: Arc<api::AppState>,
    input: CreateJob,
) -> Result<impl warp::Reply, warp::Rejection> {
    let job = state.service.create_job(input).await?;

    let res = api::Response::ok(job);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}

pub async fn get_job_result(
    state: Arc<api::AppState>,
    job_id: Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    let sleep_for = Duration::from_secs(1);

    // long polling: 5 secs
    for _ in 0..5u64 {
        let job = state.service.find_job().await?;
        match &job.output {
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
