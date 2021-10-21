use serde::{Deserialize, Serialize};
use std::{
    convert::{Into, TryFrom},
    path::PathBuf,
};
use uuid::Uuid;
use x25519_dalek::{x25519, X25519_BASEPOINT_BYTES};

use crate::Error;

pub const AGENT_CONFIG_FILE: &str = "ch12_config.json";
#[cfg(target_os = "windows")]
pub const AGENT_INSTALL_FILE: &str = "ch12_agent.exe";
#[cfg(not(target_os = "windows"))]
pub const AGENT_INSTALL_FILE: &str = "ch12_agent";
pub const INSTALL_DIRECTORY: &str = "bhr_ch12";
pub const CLIENT_IDENTITY_PUBLIC_KEY: &str = "xQ6gstFLtTbDC06LDb5dAQap+fXVG45BnRZj0L5th+M=";
pub const SINGLE_INSTANCE_IDENTIFIER: &str = "ch12_agent";

#[derive(Debug)]
pub struct Config {
    pub agent_id: Uuid,
    pub identity_public_key: ed25519_dalek::PublicKey,
    pub identity_private_key: ed25519_dalek::SecretKey,
    pub public_prekey: [u8; 32],
    pub private_prekey: [u8; 32],
    pub client_identity_public_key: ed25519_dalek::PublicKey,
}

impl TryFrom<SerializedConfig> for Config {
    type Error = Error;

    fn try_from(conf: SerializedConfig) -> Result<Config, Self::Error> {
        let agent_id = conf.agent_id;

        let identity_private_key =
            ed25519_dalek::SecretKey::from_bytes(&conf.identity_private_key)?;
        let identity_public_key: ed25519_dalek::PublicKey = (&identity_private_key).into();

        let private_prekey = conf.private_prekey;
        let public_prekey = x25519(private_prekey.clone(), X25519_BASEPOINT_BYTES);

        let client_public_key_bytes = base64::decode(CLIENT_IDENTITY_PUBLIC_KEY)?;
        let client_identity_public_key =
            ed25519_dalek::PublicKey::from_bytes(&client_public_key_bytes)?;

        Ok(Config {
            agent_id,
            identity_public_key,
            identity_private_key,
            public_prekey,
            private_prekey,
            client_identity_public_key,
        })
    }
}

impl Into<SerializedConfig> for &Config {
    fn into(self) -> SerializedConfig {
        SerializedConfig {
            agent_id: self.agent_id,
            identity_private_key: self.identity_private_key.to_bytes(),
            private_prekey: self.private_prekey,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SerializedConfig {
    pub agent_id: Uuid,
    pub identity_private_key: [u8; ed25519_dalek::SECRET_KEY_LENGTH],
    pub private_prekey: [u8; 32],
}

pub fn get_agent_config_file_path() -> Result<PathBuf, Error> {
    let mut config_file = get_agent_directory()?;

    config_file.push(AGENT_CONFIG_FILE);

    Ok(config_file)
}

pub fn get_agent_directory() -> Result<PathBuf, Error> {
    let mut data_dir = match dirs::data_dir() {
        Some(home_dir) => home_dir,
        None => return Err(Error::Internal("Error getting data directory.".to_string())),
    };

    data_dir.push(INSTALL_DIRECTORY);

    Ok(data_dir)
}

pub fn get_agent_install_target() -> Result<PathBuf, Error> {
    let mut install_target = get_agent_directory()?;
    install_target.push(AGENT_INSTALL_FILE);

    Ok(install_target)
}
