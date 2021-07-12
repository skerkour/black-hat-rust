use crate::error::Error;
use async_trait::async_trait;

pub mod cvedetails;
pub mod github;
pub mod google;

#[async_trait]
pub trait Spider {
    type Item;

    fn name(&self) -> String;
    fn start_urls(&self) -> Vec<String>;
    async fn run(&self, url: &str) -> Result<(Vec<Self::Item>, Vec<String>), Error>;
    async fn process(&self, item: Self::Item) -> Result<(), Error>;
}

// pub fn all_spiders() -> HashMap<String, Box<dyn Spider>> {
//     let spiders: Vec<Box<dyn Spider>> = vec![
//         Box::new(cvedetails::CveDetailsSpider::new()),
//         Box::new(github::GitHubSpider::new()),
//         Box::new(google::GoogleSpider::new()),
//     ];

//     return HashMap::from_iter(spiders.into_iter().map(|spider| (spider.name(), spider)));
// }
