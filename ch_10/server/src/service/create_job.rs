use super::Service;
use crate::Error;
use common::api::CreateJob;

impl Service {
    pub async fn create_job(&self, input: CreateJob) -> Result<(), Error> {
        Ok(())
    }
}
