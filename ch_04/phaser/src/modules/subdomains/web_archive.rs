use crate::{
    modules::{Module, SubdomainModule},
    Error,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use url::Url;

pub struct WebArchive {}

impl WebArchive {
    pub fn new() -> Self {
        WebArchive {}
    }
}

impl Module for WebArchive {
    fn name(&self) -> String {
        String::from("subdomains/webarchive")
    }

    fn description(&self) -> String {
        String::from("Use web.archive.org to find subdomains")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct WebArchiveResponse(Vec<Vec<String>>);

#[async_trait]
impl SubdomainModule for WebArchive {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error> {
        let url = format!("https://web.archive.org/cdx/search/cdx?matchType=domain&fl=original&output=json&collapse=urlkey&url={}", domain);
        let res = reqwest::get(&url).await?;

        if !res.status().is_success() {
            return Err(Error::InvalidHttpResponse(self.name()));
        }

        let web_archive_urls: WebArchiveResponse = match res.json().await {
            Ok(info) => info,
            Err(_) => return Err(Error::InvalidHttpResponse(self.name())),
        };

        let subdomains: HashSet<String> = web_archive_urls
            .0
            .into_iter()
            .flatten()
            .filter_map(|url| {
                Url::parse(&url)
                    .map_err(|err| {
                        log::error!("{}: error parsing url: {}", self.name(), err);
                        err
                    })
                    .ok()
            })
            .filter_map(|url| url.host_str().map(|host| host.to_string()))
            .collect();

        Ok(subdomains.into_iter().collect())
    }
}
