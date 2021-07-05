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
        String::from("Check if a .env file disclosure")
    }
}

#[async_trait]
impl HttpModule for DotEnv {
    async fn scan(&self, endpoint: &str) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/.env", &endpoint);
        let mut res = reqwest::get(&url).await?;

        if res.status().is_success() {
            return Ok(Some(HttpFinding::DotEnvFileDisclosure(url)));
        }

        Ok(None)
    }
}
