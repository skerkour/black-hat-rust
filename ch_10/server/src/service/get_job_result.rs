use super::Service;
use crate::{entities::Job, Error};

impl Service {
    pub async fn find_job(&self) -> Result<Job, Error> {
        unimplemented!();
    }
}
