use crate::{model::Subdomain, Error};
use async_trait::async_trait;
use reqwest::Client;

mod http;
mod subdomains;

pub fn all_http_modules() -> Vec<Box<dyn HttpModule>> {
    return vec![
        Box::new(http::ds_store_disclosure::DsStoreDisclosure::new()),
        Box::new(http::dotenv_disclosure::DotEnvDisclosure::new()),
        Box::new(http::directory_listing_disclosure::DirectoryListingDisclosure::new()),
        Box::new(http::traefik_dashboard_unauthenticated_access::TraefikDashboardUnauthenticatedAccess::new()),
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
}
