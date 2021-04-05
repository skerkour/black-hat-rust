use crate::{api, config::Config, Error};

pub fn run(config: Config) -> Result<(), Error> {
    let api_client = api::Client::new(config);

    let _ = api_client.list_agents()?;

    Ok(())
}
