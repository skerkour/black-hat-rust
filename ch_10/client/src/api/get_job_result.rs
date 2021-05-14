use super::Client;
use crate::{config, Error};

impl Client {
    pub fn get_job_result(&self, job_id: uuid::Uuid) -> Result<(), Error> {
        let get_job_route = format!("{}/api/jobs/{}/result", config::SERVER_URL, job_id);

        // self.http_client
        Ok(())
    }
}
