use common::api;
use uuid::Uuid;

use super::Client;
use crate::{config, Error};

impl Client {
    pub fn create_job(&self, input: api::CreateJob) -> Result<Uuid, Error> {
        let post_job_route = format!("{}/api/jobs", config::SERVER_URL);

        let res = self.http_client.post(post_job_route).json(&input).send()?;
        let api_res: api::Response<api::Job> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap().id)
    }
}
