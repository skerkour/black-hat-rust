use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;

pub struct DirectoryListing {}

impl DirectoryListing {
    pub fn new() -> Self {
        DirectoryListing {}
    }
}

impl Module for DirectoryListing {
    fn name(&self) -> String {
        String::from("http/directory_listing")
    }

    fn description(&self) -> String {
        String::from("Check for enabled directory listing, which often leak information")
    }
}

#[async_trait]
impl HttpModule for DirectoryListing {
    async fn scan(&self, endpoint: &str) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/", &endpoint);
        let mut res = reqwest::get(&url).await?;

        if res.status().is_success() {
            return Ok(Some(HttpFinding::DirectoryListingFileDisclosure(url)));
        }

        Ok(None)
    }
}
