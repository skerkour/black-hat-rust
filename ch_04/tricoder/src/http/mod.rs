use crate::Error;
use async_trait::async_trait;

#[async_trait]
pub trait HttpModule {
    async fn scan(&self, endpoint: &str) -> Result<HttpFinding, Error>;
}

pub enum HttpFinding {
    UnauthenticatedElasticsearchAccess(String),
}
