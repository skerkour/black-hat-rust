use std::collections::HashMap;

mod apple;
mod github;
mod google;

pub trait Spider {}

pub fn all_spiders() -> HashMap<String, Box<dyn Spider>> {
    let apple_spider: Box<dyn Spider> = Box::new(apple::AppleSpider::new());
    let github_spider = Box::new(github::GitHubSpider::new());
    let google_spider = Box::new(google::GoogleSpider::new());

    let mut ret = HashMap::new();
    ret.insert("apple".to_string(), apple_spider);
    ret.insert("github".to_string(), github_spider);
    ret.insert("google".to_string(), google_spider);

    return ret;
}
