use std::{collections::HashMap, iter::FromIterator};

use crate::error::Error;
use async_trait::async_trait;

mod cvedetails;
mod github;
mod google;

#[async_trait]
pub trait Spider {
    fn name(&self) -> String;
    fn start_urls(&self) -> Vec<String>;
    async fn run(&self, url: &str) -> Result<(String, Vec<String>), Error>;
}

pub fn all_spiders() -> HashMap<String, Box<dyn Spider>> {
    let spiders: Vec<Box<dyn Spider>> = vec![
        Box::new(cvedetails::CveDetailsSpider::new()),
        Box::new(github::GitHubSpider::new()),
        Box::new(google::GoogleSpider::new()),
    ];

    return HashMap::from_iter(spiders.into_iter().map(|spider| (spider.name(), spider)));
}
