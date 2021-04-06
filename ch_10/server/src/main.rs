use actix_web::{middleware, web, App, HttpServer};

mod api;
mod config;
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

    let addr = format!("0.0.0.0:{}", config.port);

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/api")
                .service(web::resource("").route(web::to(api::routes::index)))
                .service(web::resource("/commands").route(web::post().to(api::routes::commands))),
        )
    })
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}
