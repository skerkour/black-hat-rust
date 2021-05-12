use crate::Repository;
use sqlx::{Pool, Postgres};

mod create_job;
mod get_agent_job;
mod get_job_result;
mod list_agents;
mod register_agent;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: Pool<Postgres>,
}

impl Service {
    pub fn new(db: Pool<Postgres>) -> Service {
        let repo = Repository {};
        Service { db, repo }
    }
}
