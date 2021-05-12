use std::sync::Arc;
use warp::Filter;

mod api;
mod config;
mod db;
mod error;
mod repository;
mod service;
mod state;

use config::Config;
pub use error::Error;
pub use repository::Repository;
pub use service::Service;
pub use state::AppState;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "server=info");
    env_logger::init();

    let config = Config::load()?;

    let db_pool = db::connect(&config.database_url).await?;
    let service = Service::new(db_pool);
    let app_state = Arc::new(AppState::new(service));

    let api = warp::path("api");
    let api_with_state = api.and(state::with_state(app_state));

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
    let jobs = api_with_state
        .clone()
        .and(warp::path("jobs"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(api::routes::jobs);

    let routes = index
        .or(commands)
        .or(jobs)
        .with(warp::log("server"))
        .recover(api::handle_error);

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
