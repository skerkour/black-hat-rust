use crate::{model::Subdomain, Error};
use async_trait::async_trait;

mod http;
mod subdomains;

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
    async fn scan(&self, endpoint: &str) -> Result<HttpFinding, Error>;
}

pub enum HttpFinding {
    UnauthenticatedElasticsearchAccess(String),
}
