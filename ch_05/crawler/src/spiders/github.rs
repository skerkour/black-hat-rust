use crate::error::Error;
use async_trait::async_trait;

pub struct GitHubSpider {}

impl GitHubSpider {
    pub fn new() -> Self {
        GitHubSpider {}
    }
}

#[async_trait]
impl super::Spider for GitHubSpider {
    fn name(&self) -> String {
        String::from("github")
    }

    fn start_urls(&self) -> Vec<String> {
        Vec::new()
    }

    async fn run(&self, url: &str) -> Result<(String, Vec<String>), Error> {
        todo!();
    }
}
