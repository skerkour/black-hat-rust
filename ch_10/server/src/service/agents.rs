use super::Service;
use crate::Error;
use common::api::RegisterAgent;

impl Service {
    pub async fn list_agents(&self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn register_agent(&self, input: RegisterAgent) -> Result<(), Error> {
        Ok(())
    }
}
