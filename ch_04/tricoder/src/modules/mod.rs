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
        Box::new(http::KibanaUnauthenticatedAccess::new()),
        Box::new(http::GitlabOpenRegistrations::new()),
        Box::new(http::GitHeadDisclosure::new()),
        Box::new(http::GitDirectoryDisclosure::new()),
        Box::new(http::GitConfigDisclosure::new()),
        Box::new(http::EtcdUnauthenticatedAccess::new()),
        Box::new(http::Cve2017_9506::new()),
        Box::new(http::Cve2018_7600::new()),
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
    KibanaUnauthenticatedAccess(String),
    GitlabOpenRegistrations(String),
    GitHeadDisclosure(String),
    GitDirectoryDisclosure(String),
    GitConfigDisclosure(String),
    EtcdUnauthenticatedAccess(String),
    Cve2017_9506(String),
    Cve2018_7600(String),
}
