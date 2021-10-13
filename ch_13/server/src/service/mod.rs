use crate::config;
use crate::Repository;
use sqlx::{Pool, Postgres};

mod agents;
mod jobs;

pub const ENCRYPTED_JOB_MAX_SIZE: usize = 512_000; // 512k
pub const ENCRYPTED_JOB_RESULT_MAX_SIZE: usize = 2_000_000; // 2MB

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: Pool<Postgres>,
    config: config::Config,
}

impl Service {
    pub fn new(db: Pool<Postgres>, config: config::Config) -> Service {
        let repo = Repository {};
        Service { db, repo, config }
    }
}
