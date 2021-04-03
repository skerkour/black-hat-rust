use actix_web::{middleware, web, App, HttpServer};

mod config;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    ::std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/{id}/{name}").route(web::get().to(routes::index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
