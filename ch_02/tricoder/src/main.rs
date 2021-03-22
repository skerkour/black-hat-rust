use anyhow::Result;
use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use std::{env, time::Duration};

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

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(2))
        .timeout(http_timeout)
        .build()
        .expect("building HTTP client");

    let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)?
        .into_par_iter()
        .map(ports::scan_ports)
        .map(|subdomain| ports::scan_http(&http_client, subdomain))
        .collect();

    for subdomain in scan_result {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            let protocol = if port.is_http { "http" } else { "??" };
            println!("    {}: {}", port.port, protocol);
        }

        println!("");
    }

    Ok(())
}
