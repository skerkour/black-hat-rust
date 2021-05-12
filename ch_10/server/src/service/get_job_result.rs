use uuid::Uuid;

use super::Service;
use crate::{entities::Job, Error};

impl Service {
    pub async fn find_job(&self, job_id: Uuid) -> Result<Job, Error> {
        unimplemented!();
    }
}
