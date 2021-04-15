use crate::{
    common_ports::MOST_COMMON_PORTS_10,
    model::{Port, Subdomain},
};
use futures::stream;
use futures::StreamExt;
use reqwest::Client;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};

pub async fn scan_ports(subdomain: Subdomain) -> Subdomain {
    let mut ret = subdomain.clone();

    ret.open_ports = stream::iter(MOST_COMMON_PORTS_10.iter())
        .filter_map(|port| {
            let subdomain = subdomain.clone();
            async move {
                let port = scan_port(&subdomain.domain, *port).await;
                if port.is_open {
                    Some(port)
                } else {
                    None
                }
            }
        })
        .collect()
        .await;

    ret
}

pub async fn scan_http(http_client: &Client, mut subdomain: Subdomain) -> Subdomain {
    let domain = &subdomain.domain; // to avoid ownership problems

    subdomain.open_ports = stream::iter(subdomain.open_ports.into_iter())
        .then(|port| check_http(http_client, domain, port))
        .collect()
        .await;

    subdomain
}

async fn scan_port(hostname: &str, port: u16) -> Port {
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

async fn check_http(http_client: &Client, domain: &str, mut port: Port) -> Port {
    let res = http_client
        .get(&format!("http://{}:{}/", domain, port.port))
        .send()
        .await;

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
