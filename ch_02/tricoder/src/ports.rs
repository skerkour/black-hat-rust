use crate::{
    common_ports::MOST_COMMON_PORTS_10,
    model::{Port, Subdomain},
};
use rayon::prelude::*;
use reqwest::blocking::Client;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    subdomain.open_ports = MOST_COMMON_PORTS_10
        .iter()
        .map(|port| scan_port(&subdomain.domain, *port))
        .filter(|port| port.is_open) // filter closed ports
        .collect();
    subdomain
}

pub fn scan_http(http_client: &Client, mut subdomain: Subdomain) -> Subdomain {
    let domain = &subdomain.domain; // to avoid ownership problems

    subdomain.open_ports = subdomain
        .open_ports
        .into_par_iter()
        .map(|port| check_http(http_client, domain, port))
        .collect();

    subdomain
}

fn scan_port(hostname: &str, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    let socket_addresses: Vec<SocketAddr> = format!("{}:{}", hostname, port)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.len() == 0 {
        return Port {
            port: port,
            is_open: false,
            is_http: false,
        };
    }

    let is_open = if let Ok(_) = TcpStream::connect_timeout(&socket_addresses[0], timeout) {
        true
    } else {
        false
    };

    Port {
        port: port,
        is_open,
        is_http: false,
    }
}

fn check_http(http_client: &Client, domain: &str, mut port: Port) -> Port {
    let res = http_client
        .get(&format!("http://{}:{}/", domain, port.port))
        .send();

    port.is_http = match res {
        Ok(_) => true,
        Err(err) => {
            if err.is_connect() || err.is_timeout() || err.is_decode() || err.is_request() {
                false
            } else {
                true
            }
        }
    };

    port
}
