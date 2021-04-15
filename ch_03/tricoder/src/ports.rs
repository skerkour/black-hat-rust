use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};
use futures::stream;
use futures::StreamExt;
use reqwest::Client;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub async fn scan_ports(concurrency: usize, subdomain: Subdomain) -> Subdomain {
    let mut ret = subdomain.clone();
    let (input_tx, input_rx) = mpsc::channel(concurrency);
    let (output_tx, output_rx) = mpsc::channel(concurrency);

    tokio::spawn(async move {
        for port in MOST_COMMON_PORTS_100 {
            let _ = input_tx.send(*port).await;
        }
    });

    let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx);
    input_rx_stream
        .for_each_concurrent(concurrency, |port| {
            let subdomain = subdomain.clone();
            let output_tx = output_tx.clone();
            async move {
                let port = scan_port(&subdomain.domain, port).await;
                if port.is_open {
                    let _ = output_tx.send(port).await;
                }
            }
        })
        .await;
    drop(output_tx);

    let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx);
    ret.open_ports = output_rx_stream.collect().await;

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

    let is_open = if let Ok(_) =
        tokio::time::timeout(timeout, TcpStream::connect(&socket_addresses[0])).await
    {
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
