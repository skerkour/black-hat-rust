use common::api;

use super::Client;
use crate::{config, Error};

impl Client {
    pub fn list_jobs(&self) -> Result<Vec<api::Job>, Error> {
        let get_jobs_route = format!("{}/api/jobs", config::SERVER_URL);

        let res = self.http_client.get(get_jobs_route).send()?;
        let api_res: api::Response<api::JobsList> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap().jobs)
    }
}
