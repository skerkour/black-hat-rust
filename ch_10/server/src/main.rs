use api::AppState;
use std::{convert::Infallible, sync::Arc};
use warp::Filter;

mod api;
mod config;
mod db;
mod error;
mod repository;
mod service;

use config::Config;
pub use error::Error;
pub use repository::Repository;
pub use service::Service;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "server=info");
    env_logger::init();

    let config = Config::load()?;

    let db_pool = db::connect(&config.database_url).await?;
    let service = Service::new(db_pool);
    let app_state = Arc::new(api::AppState::new(service));

    let routes = routes(app_state);

    log::info!("starting server on: 0.0.0.0:{}", config.port);

    let (_addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], config.port), async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for CRTL+c");
            log::info!("Shutting down server");
        });

    server.await;

    Ok(())
}

fn routes(
    app_state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    let api = warp::path("api");
    let api_with_state = api.and(api::with_state(app_state));

    // GET /api
    let index = api
        .and(warp::path::end())
        .and(warp::get())
        .and_then(api::routes::index);

    // POST /api/commands
    let commands = api_with_state
        .clone()
        .and(warp::path("commands"))
        .and(warp::path::end())
        .and(warp::post())
        .and_then(api::routes::commands);

    // GET /api/jobs
    let get_jobs = api_with_state
        .clone()
        .and(warp::path("jobs"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(api::routes::get_jobs);

    // POST /api/agents
    let post_agents = api_with_state
        .clone()
        .and(warp::path("agents"))
        .and(warp::path::end())
        .and(warp::post())
        .and(api::json_body())
        .and_then(api::routes::post_agents);

    // GET /api/agents
    let get_agents = api_with_state
        .clone()
        .and(warp::path("agents"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(api::routes::get_agents);

    let routes = index
        .or(commands)
        .or(get_jobs)
        .or(post_agents)
        .or(get_agents)
        .with(warp::log("server"))
        .recover(api::handle_error);

    routes
}
