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

    let default_handler = warp::any().and_then(api::routes::not_found);
    let api = warp::path("api");
    let api_with_state = api.and(state::with_state(app_state));

    // GET /api
    let index = api
        .and(warp::path::end())
        .and(warp::get())
        .and_then(api::routes::index);

    // POST /api/commands
    let commands = api_with_state
        .and(warp::path("commands"))
        .and(warp::path::end())
        .and(warp::post())
        .and_then(api::routes::commands);

    let routes = index
        .or(commands)
        .or(default_handler)
        .with(warp::log("server"));

    log::info!("starting server on: 0.0.0.0:{}", config.port);
    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;

    Ok(())
}
