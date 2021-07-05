use crate::{model::Subdomain, Error};
use async_trait::async_trait;

mod http;
mod subdomains;

pub fn all_http_modules() -> Vec<Box<dyn HttpModule>> {
    return vec![
        Box::new(http::ds_store::DsStore::new()),
        Box::new(http::dotenv::DotEnv::new()),
    ];
}

pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[async_trait]
pub trait SubdomainModule: Module {
    async fn enumerate(&self) -> Result<Vec<Subdomain>, Error>;
}

#[async_trait]
pub trait HttpModule: Module {
    async fn scan(&self, endpoint: &str) -> Result<Option<HttpFinding>, Error>;
}

pub enum HttpFinding {
    UnauthenticatedElasticsearchAccess(String),
    DsStoreFileDisclosure(String),
    DotEnvFileDisclosure(String),
}
