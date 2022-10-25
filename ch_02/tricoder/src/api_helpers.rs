use std::collections::HashSet;

use serde::Deserialize;

use crate::util::configure_http_client;

#[derive(Debug, Deserialize, Clone)]
pub struct CrtShEntry {
    pub name_value: String,
}

pub fn get_subdomains_from_crt_sh(top_level_domain: &str) -> Result<HashSet<String>, anyhow::Error> {

    // Make the request and parse out just the name_value
    let entries: Vec<String> = configure_http_client()?
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", top_level_domain))
        .send()?
        .json::<Vec<CrtShEntry>>()?.into_iter().map(|CrtShEntry { name_value } | name_value ).collect();

    // clean and dedup results
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<_>>()
        })
        .filter(|subdomain: &String| !subdomain.contains('*'))
        .collect();
    subdomains.insert(top_level_domain.to_string());

    Ok(subdomains)
}