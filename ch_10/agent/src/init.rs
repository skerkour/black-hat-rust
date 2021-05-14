use crate::{consts, Error};
use common::api;
use uuid::Uuid;

pub fn init(api_client: &ureq::Agent) -> Result<Uuid, Error> {
    let saved_agent_id = get_saved_agent_id()?;

    let agent_id = match saved_agent_id {
        Some(agent_id) => agent_id,
        None => {
            let agent_id = register(api_client)?;
            save_agent_id(agent_id)?;
            agent_id
        }
    };

    Ok(agent_id)
}

pub fn register(api_client: &ureq::Agent) -> Result<Uuid, Error> {
    let register_agent_route = format!("{}/api/agents", consts::SERVER_URL);

    let api_res: api::Response<api::AgentRegistered> = api_client
        .post(register_agent_route.as_str())
        .call()?
        .into_json()?;

    let agent_id = match (api_res.data, api_res.error) {
        (Some(data), None) => Ok(data.id),
        (None, Some(err)) => Err(Error::Api(err.message)),
        (None, None) => Err(Error::Api(
            "Received invalid api response: data and error are both null.".to_string(),
        )),
        (Some(_), Some(_)) => Err(Error::Api(
            "Received invalid api response: data and error are both non null.".to_string(),
        )),
    }?;

    Ok(agent_id)
}

pub fn save_agent_id(agent_id: Uuid) -> Result<(), Error> {
    unimplemented!();
}

pub fn get_saved_agent_id() -> Result<Option<Uuid>, Error> {
    unimplemented!();
}
