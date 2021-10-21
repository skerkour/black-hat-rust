use single_instance::SingleInstance;
use std::{thread, time};

mod config;
mod error;
mod init;
mod install;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = SingleInstance::new(config::SINGLE_INSTANCE_IDENTIFIER).unwrap();

    if !instance.is_single() {
        return Ok(());
    }

    let _ = init::init_and_install()?;

    let one_sec = time::Duration::from_secs(1);
    loop {
        thread::sleep(one_sec);
    }
}
