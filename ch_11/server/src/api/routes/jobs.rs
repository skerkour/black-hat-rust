use crate::api::AppState;
use common::api;
use std::convert::TryInto;
use std::{sync::Arc, time::Duration};
use uuid::Uuid;
use warp::http::StatusCode;

pub async fn create_job(
    state: Arc<AppState>,
    input: api::CreateJob,
) -> Result<impl warp::Reply, warp::Rejection> {
    let job = state.service.create_job(input).await?;
    let job: api::Job = job.into();

    let res = api::Response::ok(job);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}

pub async fn post_job_result(
    state: Arc<AppState>,
    input: api::UpdateJobResult,
) -> Result<impl warp::Reply, warp::Rejection> {
    state.service.update_job_result(input).await?;

    let res = api::Response::ok(true);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}

pub async fn get_job_result(
    state: Arc<AppState>,
    job_id: Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    let sleep_for = Duration::from_secs(1);

    // long polling: 5 secs
    for _ in 0..5u64 {
        match state.service.get_job_result(job_id).await? {
            Some(job) => {
                let job: api::Job = job.into();
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

pub async fn get_agent_job(
    state: Arc<AppState>,
    agent_id: Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    let sleep_for = Duration::from_secs(1);

    // long polling: 5 secs
    for _ in 0..5u64 {
        match state.service.get_agent_job(agent_id).await? {
            Some(job) => {
                let agent_job = api::AgentJob {
                    id: job.id,
                    encrypted_job: job.encrypted_job,
                    ephemeral_public_key: job
                        .ephemeral_public_key
                        .try_into()
                        .expect("get_agent_job: invalid ephemeral_public_key"),
                    nonce: job.nonce.try_into().expect("get_agent_job: invalid nonce"),
                    signature: job.signature,
                };

                let res = api::Response::ok(agent_job);
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
