use single_instance::SingleInstance;
use std::{env, time::Duration};

mod clean;
mod commands;
mod config;
mod error;
mod init;
mod install;
mod run;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    if args.len() == 2 && args.nth(1).unwrap() == "clean" {
        clean::clean()?;
        return Ok(());
    }

    let instance = SingleInstance::new(config::SINGLE_INSTANCE_IDENTIFIED).unwrap();

    if !instance.is_single() {
        return Ok(());
    }

    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("ch12_agent/0.1")
        .build();

    let conf = init::init_and_install(&api_client)?;
    run::run(&api_client, conf);
}
