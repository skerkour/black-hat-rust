use futures::stream;
use futures::StreamExt;
use reqwest::Client;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::dns;
use crate::modules::Subdomain;
use crate::{modules, Error};

pub fn modules() {
    let http_modules = modules::all_http_modules();
    let subdomains_modules = modules::all_subdomains_modules();

    println!("Subdomains modules");
    for module in subdomains_modules {
        println!("   {}: {}", module.name(), module.description());
    }

    println!("HTTP modules");

    for module in http_modules {
        println!("    {}: {}", module.name(), module.description());
    }
}

pub fn scan(target: &str) -> Result<(), Error> {
    log::info!("Scanning: {}", target);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Building tokio's runtime");

    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;
    let dns_resolver = dns::new_resolver();

    let subdomains_concurrency = 20;
    let dns_concurrency = 100;
    let ports_concurrency = 200;
    // let scan_start = Instant::now();

    let subdomains_modules = modules::all_subdomains_modules();
    let http_modules = modules::all_http_modules();

    let scan_result = runtime.block_on(async move {
        // 1st step: concurrently scan subdomains
        let mut subdomains: Vec<String> = stream::iter(subdomains_modules.into_iter())
            .map(|module| async move {
                match module.enumerate(target).await {
                    Ok(new_subdomains) => Some(new_subdomains),
                    Err(err) => {
                        log::error!("subdomains/{}: {}", module.name(), err);
                        None
                    }
                }
            })
            .buffer_unordered(subdomains_concurrency)
            .filter_map(|domain| async { domain })
            .collect::<Vec<Vec<String>>>()
            .await
            .into_iter()
            .flatten()
            .collect();

        subdomains.push(target.to_string());

        // 2nd step: dedup, clean and convert results
        let subdomains: Vec<Subdomain> = HashSet::<String>::from_iter(subdomains.into_iter())
            .into_iter()
            .filter(|subdomain| subdomain.contains(target))
            .map(|domain| Subdomain {
                domain,
                open_ports: Vec::new(),
            })
            .collect();

        // 3rd step: concurrently filter unresolvable domains
        let subdomains: Vec<Subdomain> = stream::iter(subdomains.into_iter())
            .map(|domain| dns::resolves(&dns_resolver, domain))
            .buffer_unordered(dns_concurrency)
            .filter_map(|domain| async move { domain })
            .collect()
            .await;

        // 4th step: concurrently scan ports

        // 5th step: concurrently scan vulnerabilities
    });

    Ok(())
}

// let args: Vec<String> = env::args().collect();

// if args.len() != 2 {
//     return Err(Error::CliUsage.into());
// }

// let target = args[1].as_str();

// let http_timeout = Duration::from_secs(10);
// let http_client = Client::builder().timeout(http_timeout).build()?;

// let ports_concurrency = 200;
// let subdomains_concurrency = 100;
// let scan_start = Instant::now();

// let scan_result = runtime.block_on(async move {
//     let subdomains = subdomains::enumerate(&http_client, target).await?;

//     // Concurrent stream method 1: Using an Arc<Mutex<T>>
//     let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new()));

//     stream::iter(subdomains.into_iter())
//         .for_each_concurrent(subdomains_concurrency, |subdomain| {
//             let res = res.clone();
//             async move {
//                 let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
//                 res.lock().await.push(subdomain)
//             }
//         })
//         .await;

//     Ok::<_, crate::Error>(
//         Arc::try_unwrap(res)
//             .expect("Moving out from subdomains Arc")
//             .into_inner(),
//     )
// })?;

// let scan_duration = scan_start.elapsed();
// info!("Scan completed in {:?}", scan_duration);

// info!("Found {} subdomains", scan_result.len());

// for subdomain in scan_result {
//     println!("{}:", &subdomain.domain);
//     for port in &subdomain.open_ports {
//         println!("    {}: open", port.port);
//     }

//     println!("");
// }
