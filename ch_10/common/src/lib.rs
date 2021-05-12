use serde::{Deserialize, Serialize};

pub mod api;

#[derive(Deserialize, Serialize)]
pub enum ClientCommand {
    ListAgents,
}
