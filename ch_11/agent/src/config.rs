use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const SERVER_URL: &str = "http://localhost:8080";
pub const AGENT_ID_FILE: &str = "ch_11";
pub const CLIENT_IDENTITY_PUBLIC_KEY: &str = "TODO";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub agent_id: Uuid,
    pub identity_public_key: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
    pub identity_private_key: [u8; ed25519_dalek::SECRET_KEY_LENGTH],
    pub public_prekey: [u8; 32],
    pub private_prekey: [u8; 32],
}
