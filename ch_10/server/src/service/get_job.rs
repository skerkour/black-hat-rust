use super::{Job, Service};
use crate::Error;

impl Service {
    pub async fn get_job(&self) -> Result<Option<Job>, Error> {
        unimplemented!();
    }
}
