use crate::error::Error;
use async_trait::async_trait;

pub struct GitHubSpider {}

impl GitHubSpider {
    pub fn new() -> Self {
        GitHubSpider {}
    }
}

#[derive(Debug, Clone)]
pub struct GitHubItem {}

#[async_trait]
impl super::Spider for GitHubSpider {
    type Item = GitHubItem;

    fn name(&self) -> String {
        String::from("github")
    }

    fn start_urls(&self) -> Vec<String> {
        Vec::new()
    }

    async fn run(&self, url: String) -> Result<(Vec<GitHubItem>, Vec<String>), Error> {
        todo!();
    }

    async fn process(&self, item: Self::Item) -> Result<(), Error> {
        todo!()
    }
}
