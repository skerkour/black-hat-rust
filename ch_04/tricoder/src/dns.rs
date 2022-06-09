use crate::modules::Subdomain;
use std::{sync::Arc, time::Duration};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    name_server::{GenericConnection, GenericConnectionProvider, TokioRuntime},
    AsyncResolver,
};

pub type Resolver = Arc<AsyncResolver<GenericConnection, GenericConnectionProvider<TokioRuntime>>>;

pub async fn resolves(dns_resolver: &Resolver, domain: Subdomain) -> Option<Subdomain> {
    if dns_resolver.lookup_ip(domain.domain.as_str()).await.is_ok() {
        return Some(domain);
    }

    None
}

pub fn new_resolver() -> Resolver {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    let resolver = AsyncResolver::tokio(ResolverConfig::default(), opts)
        .expect("dns/new_resolver: building DNS client");

    return Arc::new(resolver);
}
