use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterAgent {
    pub id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateJob {
    pub agent: Uuid,
    pub command: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub command: String,
    pub output: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendJobOutput {
    pub id: Uuid,
    pub output: String,
}
