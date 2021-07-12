use crate::error::Error;
use async_trait::async_trait;
use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use std::time::Duration;

pub struct CveDetailsSpider {
    http_client: Client,
}

impl CveDetailsSpider {
    pub fn new() -> Self {
        let http_timeout = Duration::from_secs(5);
        let http_client = Client::builder()
            .timeout(http_timeout)
            .build()
            .expect("spiders/cvedetails: Building HTTP client");

        CveDetailsSpider { http_client }
    }
}

#[async_trait]
impl super::Spider for CveDetailsSpider {
    fn name(&self) -> String {
        String::from("cvedetails")
    }

    fn start_urls(&self) -> Vec<String> {
        vec![
            "https://www.cvedetails.com/vulnerability-list/year-2021/vulnerabilities.html"
                .to_string(),
        ]
    }

    async fn run(&self, url: &str) -> Result<(String, Vec<String>), Error> {
        let res = self.http_client.get(url).send().await?.text().await?;

        Document::from(res.as_str())
            .select(Name("a"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| println!("{}", x));

        let urls = Vec::new();
        Ok((res, urls))
    }
}
