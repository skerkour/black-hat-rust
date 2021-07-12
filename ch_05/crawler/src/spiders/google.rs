use crate::error::Error;
use async_trait::async_trait;

pub struct GoogleSpider {}

impl GoogleSpider {
    pub fn new() -> Self {
        GoogleSpider {}
    }
}

#[async_trait]
impl super::Spider for GoogleSpider {
    fn name(&self) -> String {
        String::from("google")
    }

    fn start_urls(&self) -> Vec<String> {
        Vec::new()
    }

    async fn run(&self, url: &str) -> Result<(String, Vec<String>), Error> {
        todo!();
    }
}
