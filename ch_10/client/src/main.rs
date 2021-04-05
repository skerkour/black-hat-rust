use clap::{App, SubCommand};

mod config;
mod error;

use config::Config;
pub use error::Error;

fn main() -> Result<(), anyhow::Error> {
    let conf = Config::load()?;

    let cli = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name("list").about("List all active agents"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::DisableVersion)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    Ok(())
}
