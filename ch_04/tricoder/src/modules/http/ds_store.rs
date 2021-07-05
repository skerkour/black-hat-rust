use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;

pub struct DsStore {}

impl DsStore {
    pub fn new() -> Self {
        DsStore {}
    }
}

impl Module for DsStore {
    fn name(&self) -> String {
        String::from("http/ds_store")
    }

    fn description(&self) -> String {
        String::from("Check if a .DS_Store file disclosure")
    }
}

#[async_trait]
impl HttpModule for DsStore {
    async fn scan(&self, endpoint: &str) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/.DS_Store", &endpoint);
        let mut res = reqwest::get(&url).await?;

        if !res.status().is_success() {
            return Ok(None);
        }

        if res.content_length().is_none() {
            return Err(Error::HttpResponseIsTooLarge(self.name()));
        }

        if res.content_length().unwrap() > 5_000_000 {
            // prevent DOS
            return Err(Error::HttpResponseIsTooLarge(self.name()));
        }

        let body = res.bytes().await?;
        if is_ds_store(&body.as_ref()) {
            return Ok(Some(HttpFinding::DotEnvFileDisclosure(url)));
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
