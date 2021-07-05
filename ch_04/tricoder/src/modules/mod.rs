use crate::{model::Subdomain, Error};
use async_trait::async_trait;
use reqwest::Client;

mod http;
mod subdomains;

pub fn all_http_modules() -> Vec<Box<dyn HttpModule>> {
    return vec![
        Box::new(http::DsStoreDisclosure::new()),
        Box::new(http::DotEnvDisclosure::new()),
        Box::new(http::DirectoryListingDisclosure::new()),
        Box::new(http::TraefikDashboardUnauthenticatedAccess::new()),
        Box::new(http::PrometheusDashboardUnauthenticatedAccess::new()),
    ];
}

pub fn all_subdomains_modules() -> Vec<Box<dyn SubdomainModule>> {
    return vec![];
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
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error>;
}

pub enum HttpFinding {
    UnauthenticatedElasticsearchAccess(String),
    DsStoreFileDisclosure(String),
    DotEnvFileDisclosure(String),
    DirectoryListingDisclosure(String),
    TraefikDashboardUnauthenticatedAccess(String),
    PrometheusDashboardUnauthenticatedAccess(String),
}
