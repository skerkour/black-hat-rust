use crate::error::Error;
use async_trait::async_trait;

pub struct GoogleSpider {}

impl GoogleSpider {
    pub fn new() -> Self {
        GoogleSpider {}
    }
}

#[derive(Debug, Clone)]
pub struct GoogleItem {}

#[async_trait]
impl super::Spider for GoogleSpider {
    type Item = GoogleItem;

    fn name(&self) -> String {
        String::from("google")
    }

    fn start_urls(&self) -> Vec<String> {
        Vec::new()
    }

    async fn run(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
        todo!();
    }

    async fn process(&self, item: Self::Item) -> Result<(), Error> {
        todo!()
    }
}
