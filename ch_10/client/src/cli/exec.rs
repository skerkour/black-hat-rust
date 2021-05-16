use crate::{api, Error};

pub fn run(api_client: &api::Client, agent_id: &str, command: &str) -> Result<(), Error> {
    let _ = api_client.list_agents()?;

    Ok(())
}
