use crate::Error;

#[derive(Clone, Debug)]
pub struct Config {
    pub token: String,
    pub server_url: String,
}

const ENV_SERVER_URL: &str = "SERVER_URL";
const ENV_TOKEN: &str = "TOKEN";

impl Config {
    pub fn load() -> Result<Config, Error> {
        let token = std::env::var(ENV_TOKEN).map_err(|_| env_not_found(ENV_TOKEN))?;

        let server_url =
            std::env::var(ENV_SERVER_URL).map_err(|_| env_not_found(ENV_SERVER_URL))?;

        Ok(Config { token, server_url })
    }
}

fn env_not_found(var: &str) -> Error {
    Error::NotFound(format!("config: {} env var not found", var))
}
