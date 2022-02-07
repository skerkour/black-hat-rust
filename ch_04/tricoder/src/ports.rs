use crate::{
    common_ports::MOST_COMMON_PORTS,
    modules::{Port, Subdomain},
};
use futures::{stream, StreamExt};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::net::TcpStream;

pub async fn scan_ports(concurrency: usize, mut subdomain: Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.len() == 0 {
        return subdomain;
    }

    let socket_address = socket_addresses[0];

    subdomain.open_ports = stream::iter(MOST_COMMON_PORTS.into_iter())
        .map(|port| async move {
            let port = scan_port(socket_address, *port).await;
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

async fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_address.set_port(port);

    let is_open = matches!(
        tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await,
        Ok(Ok(_)),
    );

    Port {
        port: port,
        is_open,
        findings: Vec::new(),
    }
}
