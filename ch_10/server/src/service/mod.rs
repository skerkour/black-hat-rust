use crate::Repository;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

mod get_job;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {}
