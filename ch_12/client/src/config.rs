use crate::Error;

pub const SERVER_URL: &str = "http://localhost:8080";
pub const IDENTITY_PRIVATE_KEY: &str = "wToLgDfjCxFijRA+YKi6T9j7bTc/4grwoTRJZJs5DU8=";

#[derive(Debug)]
pub struct Config {
    pub identity_public_key: ed25519_dalek::PublicKey,
    pub identity_private_key: ed25519_dalek::SecretKey,
}

impl Config {
    pub fn load() -> Result<Config, Error> {
        let private_key_bytes = base64::decode(IDENTITY_PRIVATE_KEY)?;

        let identity_private_key = ed25519_dalek::SecretKey::from_bytes(&private_key_bytes)?;
        let identity_public_key: ed25519_dalek::PublicKey = (&identity_private_key).into();

        Ok(Config {
            identity_private_key,
            identity_public_key,
        })
    }
}
