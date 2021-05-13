use super::Repository;
use crate::{entities::Agent, Error};
use sqlx::{Pool, Postgres};

impl Repository {
    pub async fn create_agent(&self, db: &Pool<Postgres>, agent: &Agent) -> Result<(), Error> {
        Ok(())
    }

    pub async fn update_agent(&self, db: &Pool<Postgres>, agent: &Agent) -> Result<(), Error> {
        Ok(())
    }
}
