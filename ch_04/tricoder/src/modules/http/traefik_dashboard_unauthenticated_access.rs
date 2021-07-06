use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;
use reqwest::Client;

pub struct TraefikDashboardUnauthenticatedAccess {}

impl TraefikDashboardUnauthenticatedAccess {
    pub fn new() -> Self {
        TraefikDashboardUnauthenticatedAccess {}
    }
}

impl Module for TraefikDashboardUnauthenticatedAccess {
    fn name(&self) -> String {
        String::from("http/traefik_dashboard_unauthenticated_access")
    }

    fn description(&self) -> String {
        String::from("Check for Traefik Dashboard Unauthenticated Access")
    }
}

#[async_trait]
impl HttpModule for TraefikDashboardUnauthenticatedAccess {
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
        if (body.contains(r#"ng-app="traefik""#)
            && body.contains(r#"href="https://docs.traefik.io""#)
            && body.contains(r#"href="https://traefik.io""#))
            || body
                .contains(r#"fixed-top"><head><meta charset="utf-8"><title>Traefik</title><base"#)
        {
            return Ok(Some(HttpFinding::TraefikDashboardUnauthenticatedAccess(
                url,
            )));
        }

        Ok(None)
    }
}
