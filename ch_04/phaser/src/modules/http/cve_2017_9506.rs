use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;
use reqwest::Client;

pub struct Cve2017_9506 {}

impl Cve2017_9506 {
    pub fn new() -> Self {
        Cve2017_9506 {}
    }
}

impl Module for Cve2017_9506 {
    fn name(&self) -> String {
        String::from("http/cve_2017_9506")
    }

    fn description(&self) -> String {
        String::from("Check for CVE-2017-9506 (SSRF)")
    }
}

#[async_trait]
impl HttpModule for Cve2017_9506 {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!(
            "{}/plugins/servlet/oauth/users/icon-uri?consumerUri=https://google.com/robots.txt",
            &endpoint
        );
        let res = http_client.get(&url).send().await?;

        if !res.status().is_success() {
            return Ok(None);
        }

        let body = res.text().await?;
        if body.contains("user-agent: *") && body.contains("disallow") {
            return Ok(Some(HttpFinding::Cve2017_9506(url)));
        }

        Ok(None)
    }
}
