use crate::{
    common_ports::MOST_COMMON_PORTS,
    modules::{Port, Subdomain},
};
use futures::{stream, StreamExt};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::net::TcpStream;

pub async fn scan_ports(concurrency: usize, mut subdomain: Subdomain) -> Subdomain {
    let hostname = &subdomain.domain.clone();

    subdomain.open_ports = stream::iter(MOST_COMMON_PORTS.into_iter())
        .map(|port| async move {
            let port = scan_port(hostname, *port).await;
            if port.is_open {
                return Some(port);
            }
            None
        })
        .buffer_unordered(concurrency)
        .filter_map(|port| async { port })
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
            findings: Vec::new(),
        };
    }

    let is_open =
        match tokio::time::timeout(timeout, TcpStream::connect(&socket_addresses[0])).await {
            Ok(Ok(_)) => true,
            _ => false,
        };

    Port {
        port: port,
        is_open,
        findings: Vec::new(),
    }
}
