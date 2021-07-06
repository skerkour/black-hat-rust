use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;
use reqwest::Client;

pub struct DsStoreDisclosure {}

impl DsStoreDisclosure {
    pub fn new() -> Self {
        DsStoreDisclosure {}
    }
}

impl Module for DsStoreDisclosure {
    fn name(&self) -> String {
        String::from("http/ds_store")
    }

    fn description(&self) -> String {
        String::from("Check if a .DS_Store file disclosure")
    }
}

#[async_trait]
impl HttpModule for DsStoreDisclosure {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/.DS_Store", &endpoint);
        let res = http_client.get(&url).send().await?;

        if !res.status().is_success() {
            return Ok(None);
        }

        let body = res.bytes().await?;
        if is_ds_store(&body.as_ref()) {
            return Ok(Some(HttpFinding::DsStoreFileDisclosure(url)));
        }

        Ok(None)
    }
}

fn is_ds_store(content: &[u8]) -> bool {
    if content.len() < 8 {
        return false;
    }

    let signature = [0x0, 0x0, 0x0, 0x1, 0x42, 0x75, 0x64, 0x31];

    return content[0..8] == signature;
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_ds_store() {
        let body = "testtesttest";
        let body2 = [
            0x00, 0x00, 0x00, 0x01, 0x42, 0x75, 0x64, 0x31, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00,
            0x08, 0x0,
        ];

        assert_eq!(false, super::is_ds_store(body.as_bytes()));
        assert_eq!(true, super::is_ds_store(&body2));
    }
}
