use single_instance::SingleInstance;
use std::{thread, time};

mod config;
mod error;
mod install;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = SingleInstance::new(config::SINGLE_INSTANCE_IDENTIFIED).unwrap();
    if !instance.is_single() {
        return Ok(());
    }

    install::install()?;

    let one_sec = time::Duration::from_secs(1);
    loop {
        thread::sleep(one_sec);
    }
}
