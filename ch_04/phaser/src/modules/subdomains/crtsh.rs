// use crate::{
//     model::{CrtShEntry, Subdomain},
//     Error,
// };
// use futures::stream;
// use futures::StreamExt;
// use reqwest::Client;
// use std::{collections::HashSet, time::Duration};
// use trust_dns_resolver::{
//     config::{ResolverConfig, ResolverOpts},
//     name_server::{GenericConnection, GenericConnectionProvider, TokioRuntime},
//     AsyncResolver,
// };

// type DnsResolver = AsyncResolver<GenericConnection, GenericConnectionProvider<TokioRuntime>>;

// pub async fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
//     let entries: Vec<CrtShEntry> = http_client
//         .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
//         .send()
//         .await?
//         .json()
//         .await?;

//     let dns_resolver = AsyncResolver::tokio(
//         ResolverConfig::default(),
//         ResolverOpts {
//             timeout: Duration::from_secs(4),
//             ..Default::default()
//         },
//     )
//     .expect("subdomain resolver: building DNS client");

//     // clean and dedup results
//     let mut subdomains: HashSet<String> = entries
//         .into_iter()
//         .map(|entry| {
//             entry
//                 .name_value
//                 .split("\n")
//                 .map(|subdomain| subdomain.trim().to_string())
//                 .collect::<Vec<String>>()
//         })
//         .flatten()
//         .filter(|subdomain: &String| subdomain != target)
//         .filter(|subdomain: &String| !subdomain.contains("*"))
//         .collect();
//     subdomains.insert(target.to_string());

//     let subdomains: Vec<Subdomain> = stream::iter(subdomains.into_iter())
//         .map(|domain| Subdomain {
//             domain,
//             open_ports: Vec::new(),
//         })
//         .filter_map(|subdomain| {
//             let dns_resolver = dns_resolver.clone();
//             async move {
//                 if resolves(&dns_resolver, &subdomain).await {
//                     Some(subdomain)
//                 } else {
//                     None
//                 }
//             }
//         })
//         .collect()
//         .await;

//     Ok(subdomains)
// }

use crate::{
    modules::{Module, SubdomainModule},
    Error,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub struct Crtsh {}

impl Crtsh {
    pub fn new() -> Self {
        Crtsh {}
    }
}

impl Module for Crtsh {
    fn name(&self) -> String {
        String::from("subdomains/crtsh")
    }

    fn description(&self) -> String {
        String::from("Use crt.sh/ to find subdomains")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CrtShEntry {
    name_value: String,
}

#[async_trait]
impl SubdomainModule for Crtsh {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error> {
        let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
        let res = reqwest::get(&url).await?;

        if !res.status().is_success() {
            return Err(Error::InvalidHttpResponse(self.name()));
        }

        let crtsh_entries: Vec<CrtShEntry> = match res.json().await {
            Ok(info) => info,
            Err(_) => return Err(Error::InvalidHttpResponse(self.name())),
        };

        // clean and dedup results
        let subdomains: HashSet<String> = crtsh_entries
            .into_iter()
            .map(|entry| {
                entry
                    .name_value
                    .split("\n")
                    .map(|subdomain| subdomain.trim().to_string())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .filter(|subdomain: &String| !subdomain.contains("*"))
            .collect();

        Ok(subdomains.into_iter().collect())
    }
}
