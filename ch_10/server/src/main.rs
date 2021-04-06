use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;

mod api;
mod config;
mod db;
mod error;
mod state;

use config::Config;
pub use error::Error;
pub use state::ServerState;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    ::std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let config = Config::load()?;

    let db_pool = db::connect(&config.database_url).await?;
    let addr = format!("0.0.0.0:{}", config.port);

    let app_state = Arc::new(ServerState::new(db_pool));

    HttpServer::new(move || {
        App::new()
            .data(Arc::clone(&app_state))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(web::resource("").route(web::to(api::routes::index)))
                    .service(
                        web::resource("/commands").route(web::post().to(api::routes::commands)),
                    ),
            )
    })
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}
