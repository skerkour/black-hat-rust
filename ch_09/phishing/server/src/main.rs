use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get_service, post},
    Router,
};
use clap::Arg;
use log::info;
use std::{io, net::SocketAddr};
use tower_http::services::{ServeDir, ServeFile};

mod api;
mod db;
mod error;
pub use error::Error;
use sqlx::SqlitePool;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Error> {
    let cli_matches = clap::Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .help("Port to listen to")
                .default_value("8080"),
        )
        .arg(
            Arg::new("directory")
                .long("directory")
                .short('d')
                .help("Directory to server")
                .default_value("public"),
        )
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

    let app = Router::new()
        .route("/api/login", post(api::login))
        .fallback(
            get_service(
                ServeDir::new(&public_dir).fallback(ServeFile::new(public_dir + "/index.html")),
            )
            .handle_error(handle_error),
        )
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error...")
}
