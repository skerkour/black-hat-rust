use super::Client;
use crate::{config, Error};

impl Client {
    pub fn list_agents(&self) -> Result<(), Error> {
        let get_agents_route = format!("{}/api/agents", config::SERVER_URL);

        // self.http_client
        Ok(())
    }
}
