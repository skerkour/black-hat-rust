use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;
use reqwest::Client;

pub struct EtcdUnauthenticatedAccess {}

impl EtcdUnauthenticatedAccess {
    pub fn new() -> Self {
        EtcdUnauthenticatedAccess {}
    }
}

impl Module for EtcdUnauthenticatedAccess {
    fn name(&self) -> String {
        String::from("http/etc_unauthenticated_access")
    }

    fn description(&self) -> String {
        String::from("Check for CoreOS' etcd Unauthenticated Access")
    }
}

#[async_trait]
impl HttpModule for EtcdUnauthenticatedAccess {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/version", &endpoint);
        let res = http_client.get(&url).send().await?;

        if !res.status().is_success() {
            return Ok(None);
        }

        let body = res.text().await?;
        if body.contains(r#""etcdserver""#)
            && body.contains(r#""etcdcluster""#)
            && body.chars().count() < 200
        {
            return Ok(Some(HttpFinding::EtcdUnauthenticatedAccess(url)));
        }

        Ok(None)
    }
}
