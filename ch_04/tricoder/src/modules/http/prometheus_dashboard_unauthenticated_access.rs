use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;
use reqwest::Client;

pub struct PrometheusDashboardUnauthenticatedAccess {}

impl PrometheusDashboardUnauthenticatedAccess {
    pub fn new() -> Self {
        PrometheusDashboardUnauthenticatedAccess {}
    }
}

impl Module for PrometheusDashboardUnauthenticatedAccess {
    fn name(&self) -> String {
        String::from("http/prometheus_dashboard_unauthenticated_access")
    }

    fn description(&self) -> String {
        String::from("Check for Prometheus Dashboard Unauthenticated Access")
    }
}

#[async_trait]
impl HttpModule for PrometheusDashboardUnauthenticatedAccess {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}", &endpoint);
        let res = http_client.get(&url).send().await?;

        if !res.status().is_success() {
            return Ok(None);
        }

        let body = res.text().await?;
        if body
            .contains(r#"<title>Prometheus Time Series Collection and Processing Server</title>"#)
        {
            return Ok(Some(HttpFinding::PrometheusDashboardUnauthenticatedAccess(
                url,
            )));
        }

        Ok(None)
    }
}
