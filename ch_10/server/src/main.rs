use actix_web::{middleware, web, App, HttpServer};

mod config;
mod error;
mod routes;

use config::Config;
pub use error::Error;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    ::std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let config = Config::load()?;

    let addr = format!("0.0.0.0:{}", config.port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/{id}/{name}").route(web::get().to(routes::index)))
    })
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}
