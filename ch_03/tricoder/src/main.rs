use anyhow::Result;
use futures::{stream, StreamExt};
use reqwest::Client;
use std::{
    env,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

mod error;
pub use error::Error;
mod model;
mod ports;
mod subdomains;
use model::Subdomain;
mod common_ports;

fn main() -> Result<()> {
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

        // Comcurrent stream ethod 1: Using an Arc<Mutex<T>>
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
    println!("Scan completed in {:?}", scan_duration);

    for subdomain in scan_result {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            println!("    {}: open", port.port);
        }

        println!("");
    }

    Ok(())
}
