use std::time::Duration;

mod consts;
mod error;
mod init;
mod run;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("ch_10_agent/0.1")
        .build();

    let agent_id = init::init(&api_client)?;
    run::run(&api_client, agent_id);
}
