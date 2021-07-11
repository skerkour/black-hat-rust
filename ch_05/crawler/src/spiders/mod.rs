use std::{collections::HashMap, iter::FromIterator};

mod cvedetails;
mod github;
mod google;

pub trait Spider {
    fn name(&self) -> String;
}

pub fn all_spiders() -> HashMap<String, Box<dyn Spider>> {
    let spiders: Vec<Box<dyn Spider>> = vec![
        Box::new(cvedetails::CveDetailsSpider::new()),
        Box::new(github::GitHubSpider::new()),
        Box::new(google::GoogleSpider::new()),
    ];

    return HashMap::from_iter(spiders.into_iter().map(|spider| (spider.name(), spider)));
}
