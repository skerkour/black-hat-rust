use api_helpers::get_subdomains_from_crt_sh;
use config::{HTTP_TIMEOUT, MAX_REDIRECTS, NUM_THREADS};
use rayon::{prelude::*, ThreadPool, ThreadPoolBuildError};
use reqwest::{blocking::Client, redirect};
use serde::Deserialize;
use std::{collections::HashSet, env, time::Duration};

mod error;
pub use error::Error;
mod scanned_subdomain;
use scanned_subdomain::{ScannedSubdomain};
mod common_ports;
mod config;
mod util;
mod api_helpers;

fn main() -> Result<(), anyhow::Error> {
    let top_level_domain = parse_cli_input()?;

    // Make an api request to crt.sh to get the subdomains
    let subdomains = get_subdomains_from_crt_sh(&top_level_domain)?;

    // For each subdomain, look for open ports and built a list of subdomains
    // and their open ports, excluding the ones that fail to resolve.
    let scanned_subdomains = 
        subdomains
            .into_iter()
            .filter_map(|sd| ScannedSubdomain::try_from(sd).ok())
            .collect::<Vec<_>>();

    // Print out the discovered subdomains and their open ports
    for sd in scanned_subdomains {
        println!("{}\n", sd);
    }

    Ok(())
}

fn parse_cli_input() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err(Error::CliUsage.into())
    } else {
        Ok(args[1].clone())
    }
}
