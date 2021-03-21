use crate::{
    common_ports::MOST_COMMON_PORTS,
    model::{Port, Subdomain},
};
use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    subdomain.open_ports = MOST_COMMON_PORTS.iter().map(scan_port).collect();
    subdomain
}

pub fn scan_http(mut subdomain: Subdomain) -> Subdomain {
    let http_client = Client::builder()
    .redirect(redirect::Policy::limited(1))
    .build().expect("http scanner: building HTTP client");

    let domain = &subdomain.domain; // to avoid ownership problems

    subdomain.open_ports = subdomain
        .open_ports
        .into_par_iter()
        .map(|port| check_http(&http_client, domain, port))
        .collect();

    subdomain
}

fn scan_port(port: &u16) -> Port {
    todo!();
}

fn check_http(http_client: &Client, domain: &str, mut port: Port) -> Port {
    let res = http_client
        .get(&format!("http://{}:{}/", domain, port.port))
        .send();

    port.is_http = match res {
        Ok(_) => true,
        Err(err) => {
            if err.is_connect() || err.is_timeout() {
                false
            } else {
                true
            }
        }
    };

    port
}
