use anyhow::Result;
use clap::{App, Arg, SubCommand};
use futures::{stream, StreamExt};
use log::info;
use reqwest::Client;
use std::{
    env,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

mod common_ports;
mod error;
mod http;
mod model;
mod ports;
mod subdomains;
pub use error::Error;
use model::Subdomain;

fn main() -> Result<()> {
    let cli = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name("modules").about("List all modules"))
        .subcommand(
            SubCommand::with_name("scan").about("Scan a target").arg(
                Arg::with_name("target")
                    .help("The domain name to scan")
                    .required(true)
                    .index(1),
            ),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Building tokio's runtime");

    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;

    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let scan_start = Instant::now();

    let scan_result = runtime.block_on(async move {
        let subdomains = subdomains::enumerate(&http_client, target).await?;

        // Concurrent stream method 1: Using an Arc<Mutex<T>>
        let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new()));

        stream::iter(subdomains.into_iter())
            .for_each_concurrent(subdomains_concurrency, |subdomain| {
                let res = res.clone();
                async move {
                    let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
                    res.lock().await.push(subdomain)
                }
            })
            .await;

        Ok::<_, crate::Error>(
            Arc::try_unwrap(res)
                .expect("Moving out from subdomains Arc")
                .into_inner(),
        )
    })?;

    let scan_duration = scan_start.elapsed();
    info!("Scan completed in {:?}", scan_duration);

    info!("Found {} subdomains", scan_result.len());

    // for subdomain in scan_result {
    //     println!("{}:", &subdomain.domain);
    //     for port in &subdomain.open_ports {
    //         println!("    {}: open", port.port);
    //     }

    //     println!("");
    // }

    Ok(())
}
