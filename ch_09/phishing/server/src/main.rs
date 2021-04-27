use actix_files::NamedFile;
use actix_web::{middleware, web, App, HttpServer};
use clap::Arg;
use log::info;
use std::sync::Arc;

mod api;
mod db;
mod error;
pub use error::Error;
use sqlx::SqlitePool;

async fn index() -> Result<NamedFile, actix_web::Error> {
    Ok(NamedFile::open("public/index.html")?)
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let cli_matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .help("Port to listen to")
                .default_value("8080"),
        )
        .arg(
            Arg::with_name("directory")
                .long("directory")
                .short("d")
                .help("Directory to server")
                .default_value("public"),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    std::env::set_var("RUST_LOG", "actix_web=info,server=info");
    env_logger::init();

    let pool = db::connect(db::DATABASE_URL).await?;
    let port = cli_matches.value_of("port").unwrap().parse::<u16>()?;
    let public_dir = cli_matches.value_of("directory").unwrap().to_string();

    run_server(pool, port, public_dir).await?;

    Ok(())
}

async fn run_server(pool: SqlitePool, port: u16, public_dir: String) -> Result<(), Error> {
    let endpoint = format!("0.0.0.0:{}", port);

    info!("Starting server. port={}, directory={}", port, &public_dir);

    let pool = Arc::new(pool);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource(common::api::routes::LOGIN).route(web::post().to(api::login)))
            .service(
                // serve webapp
                actix_files::Files::new("/", &public_dir)
                    .index_file("index.html")
                    .prefer_utf8(true)
                    .default_handler(web::route().to(index)),
            )
            .default_service(
                // 404
                web::resource("").to(index),
            )
    })
    .bind(&endpoint)?
    .run()
    .await?;
    Ok(())
}
