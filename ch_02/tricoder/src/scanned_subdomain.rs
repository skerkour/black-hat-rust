use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

use rayon::prelude::*;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

use crate::{common_ports::MOST_COMMON_PORTS_100, config::RESOLUTION_TIMEOUT};
use crate::{config::SCAN_TIMEOUT, util};

#[derive(Debug, Clone)]
pub struct ScannedSubdomain {
    pub subdomain: String,
    pub open_ports: Vec<u16>,
}

fn verify_domain_resolves(subdomain: &str) -> Result<(), anyhow::Error> {
    let mut opts = ResolverOpts::default();
    opts.timeout = RESOLUTION_TIMEOUT;
    let dns_resolver = Resolver::new(ResolverConfig::default(), opts)?;
    dns_resolver.lookup_ip(subdomain)?;
    Ok(())
}

fn get_open_port(mut addr: SocketAddr, port: u16) -> Option<u16> {
    addr.set_port(port);
    TcpStream::connect_timeout(&addr, SCAN_TIMEOUT)
        .is_ok()
        .then_some(port)
}

impl TryFrom<String> for ScannedSubdomain {
    type Error = anyhow::Error;

    fn try_from(subdomain: String) -> Result<Self, Self::Error> {
        // First, make sure the subdomain resolves
        verify_domain_resolves(&subdomain)?;

        // Get the socket addresses
        let socket_addrs: Vec<SocketAddr> =
            format!("{}:1024", subdomain).to_socket_addrs()?.collect();

        // Scan each port (concurrently)
        let pool = util::configure_threadpool()?;
        let open_ports = pool.install(|| {
            (!socket_addrs.is_empty())
                .then(|| {
                    MOST_COMMON_PORTS_100
                        .into_par_iter()
                        .filter_map(|port| get_open_port(socket_addrs[0], *port))
                        .collect()
                })
                .unwrap_or_default() // I prefer unwrap_or(vec![]) for explicitness, but cargo-clippy doesn't like it.
        });

        Ok(Self {
            subdomain,
            open_ports,
        })
    }
}

impl std::fmt::Display for ScannedSubdomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str(&self.subdomain);
        self.open_ports.iter().for_each(|op| {
            buffer.push_str(&format!("\n   {}", op));
        });
        write!(f, "{}", buffer)
    }
}
