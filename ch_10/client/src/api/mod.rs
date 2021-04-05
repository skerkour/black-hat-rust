use crate::config::Config;
use reqwest::redirect;
use std::time::Duration;

mod list_agents;

#[derive(Debug, Clone)]
pub struct Client {
    pub http_client: reqwest::blocking::Client,
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Client {
        let http_timeout = Duration::from_secs(5);
        let http_client = reqwest::blocking::Client::builder()
            .redirect(redirect::Policy::limited(4))
            .timeout(http_timeout)
            .build()
            .expect("api: Building HTTP client");

        Client {
            http_client,
            config,
        }
    }
}
