use crate::{api, Error};

pub fn run(api_client: &api::Client) -> Result<(), Error> {
    let _ = api_client.list_agents()?;

    Ok(())
}
