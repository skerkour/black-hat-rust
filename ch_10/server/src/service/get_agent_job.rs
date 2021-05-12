use super::Service;
use crate::{entities::Job, Error};

impl Service {
    pub async fn get_agent_job(&self) -> Result<Option<Job>, Error> {
        Ok(None)
    }
}
