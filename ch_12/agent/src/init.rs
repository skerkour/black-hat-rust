use crate::{config, install, Error};
use common::crypto;
use rand::RngCore;
use std::{convert::TryInto, fs};
use x25519_dalek::{x25519, X25519_BASEPOINT_BYTES};

pub fn init_and_install() -> Result<config::Config, Error> {
    let saved_agent_id = get_saved_agent_config()?;

    let conf = match saved_agent_id {
        Some(agent_id) => agent_id,
        None => {
            install::install()?;
            let conf = register()?;
            save_agent_config(&conf)?;
            conf
        }
    };

    Ok(conf)
}

pub fn register() -> Result<config::Config, Error> {
    let mut rand_generator = rand::rngs::OsRng {};

    let identity_keypair = ed25519_dalek::Keypair::generate(&mut rand_generator);

    let mut private_prekey = [0u8; crypto::X25519_PRIVATE_KEY_SIZE];
    rand_generator.fill_bytes(&mut private_prekey);
    let public_prekey = x25519(private_prekey.clone(), X25519_BASEPOINT_BYTES);

    let client_public_key_bytes = base64::decode(config::CLIENT_IDENTITY_PUBLIC_KEY)?;
    let client_identity_public_key =
        ed25519_dalek::PublicKey::from_bytes(&client_public_key_bytes)?;

    let conf = config::Config {
        agent_id: uuid::Uuid::new_v4(),
        identity_public_key: identity_keypair.public,
        identity_private_key: identity_keypair.secret,
        public_prekey,
        private_prekey,
        client_identity_public_key,
    };

    Ok(conf)
}

pub fn save_agent_config(conf: &config::Config) -> Result<(), Error> {
    let agent_config_file = config::get_agent_config_file_path()?;

    let serialized_conf: config::SerializedConfig = conf.into();
    let config_json = serde_json::to_string(&serialized_conf)?;

    fs::write(agent_config_file, config_json.as_bytes())?;

    Ok(())
}

pub fn get_saved_agent_config() -> Result<Option<config::Config>, Error> {
    let agent_id_file = config::get_agent_config_file_path()?;

    if agent_id_file.exists() {
        let agent_file_content = fs::read(agent_id_file)?;

        let serialized_conf: config::SerializedConfig =
            serde_json::from_slice(&agent_file_content)?;
        let conf = serialized_conf.try_into()?;
        Ok(Some(conf))
    } else {
        Ok(None)
    }
}
