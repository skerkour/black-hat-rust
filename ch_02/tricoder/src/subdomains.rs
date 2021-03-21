use crate::{model::Subdomain, Error};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

pub fn enumerate(target: &str) -> Result<Vec<Subdomain>, Error> {
    // find subdomains from crt.sh
    // generate from most common subdomains
    // filter
    Ok(Vec::new())
}

pub fn resolves(domain: &Subdomain) -> bool {
    let dns_resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).expect("subdomain resolver: building DNS client");
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}
