use super::Client;
use crate::{config, Error};
use common::api;

impl Client {
    pub fn list_agents(&self) -> Result<Vec<api::Agent>, Error> {
        let get_agents_route = format!("{}/api/agents", config::SERVER_URL);

        let res = self.http_client.get(get_agents_route).send()?;
        let api_res: api::Response<api::AgentsList> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap().agents)
    }
}
