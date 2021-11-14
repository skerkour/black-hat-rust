use clap::Arg;
use log::info;
use std::{path::Path, sync::Arc};
use warp::Filter;

mod api;
mod db;
mod error;
pub use error::Error;
use sqlx::SqlitePool;

#[tokio::main(flavor = "multi_thread")]
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

    std::env::set_var("RUST_LOG", "server=info");
    env_logger::init();

    let pool = db::connect(db::DATABASE_URL).await?;
    let port = cli_matches.value_of("port").unwrap().parse::<u16>()?;
    let public_dir = cli_matches.value_of("directory").unwrap().to_string();

    run_server(pool, port, public_dir).await?;

    Ok(())
}

async fn run_server(pool: SqlitePool, port: u16, public_dir: String) -> Result<(), Error> {
    info!("Starting server. port={}, directory={}", port, &public_dir);

    let pool = Arc::new(pool);
    let index_path = Path::new(&public_dir)
        .join("index.html")
        .into_os_string()
        .into_string()
        .unwrap();

    let index = warp::any().and(warp::fs::file(index_path));
    let files = warp::any().and(warp::fs::dir(public_dir));
    let login = warp::path("api")
        .and(warp::path("login"))
        .and(warp::post())
        .and(api::json_body())
        .and(api::with_db(pool))
        .and_then(api::login);

    let routes = files.or(login).or(index).with(warp::log("server"));
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
