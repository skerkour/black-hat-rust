use super::Client;
use crate::{config, Error};

impl Client {
    pub fn list_jobs(&self) -> Result<(), Error> {
        let get_jobs_route = format!("{}/api/jobs", config::SERVER_URL);

        // self.http_client
        Ok(())
    }
}
