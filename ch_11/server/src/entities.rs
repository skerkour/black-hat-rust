use chrono::{DateTime, Utc};
use common::api;
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
    pub command: String,
    pub args: Json<Vec<String>>,
    pub output: Option<String>,

    pub agent_id: Uuid,
}

impl Into<api::Job> for Job {
    fn into(self) -> api::Job {
        api::Job {
            id: self.id,
            created_at: self.created_at,
            executed_at: self.executed_at,
            command: self.command,
            args: self.args.0,
            output: self.output,
            agent_id: self.agent_id,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

impl Into<api::Agent> for Agent {
    fn into(self) -> api::Agent {
        api::Agent {
            id: self.id,
            created_at: self.created_at,
            last_seen_at: self.last_seen_at,
        }
    }
}
