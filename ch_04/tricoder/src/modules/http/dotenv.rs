use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;

pub struct DotEnv {}

impl DotEnv {
    pub fn new() -> Self {
        DotEnv {}
    }
}

impl Module for DotEnv {
    fn name(&self) -> String {
        String::from("http/dotenv")
    }

    fn description(&self) -> String {
        String::from("Check if a .env file is present")
    }
}

#[async_trait]
impl HttpModule for DotEnv {
    async fn scan(&self, endpoint: &str) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/.env", &endpoint);
        let mut res = reqwest::get(&url).await?;

        if res.content_length().is_none() {
            return Err(Error::HttpResponseIsTooLarge(self.name()));
        }

        if res.content_length().unwrap() > 5_000_000 {
            // prevent DOS
            return Err(Error::HttpResponseIsTooLarge(self.name()));
        }

        Ok(None)
    }
}
