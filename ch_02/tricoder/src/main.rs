use anyhow::Result;
use rayon::prelude::*;
use std::env;

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

    let scan_result: Vec<Subdomain> = subdomains::enumerate(target)?
        .into_par_iter()
        .map(ports::scan_ports)
        .map(ports::scan_http)
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
