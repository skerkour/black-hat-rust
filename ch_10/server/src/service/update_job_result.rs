use super::Service;
use crate::Error;
use common::api::UpdateJobResult;

impl Service {
    pub async fn update_job_result(&self, input: UpdateJobResult) -> Result<(), Error> {
        Ok(())
    }
}
