use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ClientCommand {
    ListAgents,
}
