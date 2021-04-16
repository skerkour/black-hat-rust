use anyhow::Result;
use futures::{stream, StreamExt};
use reqwest::{redirect, Client};
use std::{env, sync::Arc, time::Duration};
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

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;
    let ports_concurrency = 200;
    let subdomains_concurrency = 100;

    let scan_result = runtime.block_on(async move {
        let subdomains = subdomains::enumerate(&http_client, target).await?;

        let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new()));

        stream::iter(subdomains.into_iter())
            .for_each_concurrent(subdomains_concurrency, |subdomain| {
                let http_client = http_client.clone();
                let res = res.clone();
                async move {
                    let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
                    let subdomain = ports::scan_http(&http_client, subdomain).await;
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

    for subdomain in scan_result {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            let protocol = if port.is_http { "http" } else { "tcp" };
            println!("    {}: {}", port.port, protocol);
        }

        println!("");
    }

    Ok(())
}
