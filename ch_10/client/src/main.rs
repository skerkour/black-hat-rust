use clap::{App, SubCommand};

mod cli;
mod config;
mod error;

use config::Config;
pub use error::Error;

fn main() -> Result<(), anyhow::Error> {
    let cli = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name(cli::TOKEN).about("Generates a secure token"))
        .subcommand(SubCommand::with_name(cli::LIST).about("List all active agents"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::DisableVersion)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    if let Some(_) = cli.subcommand_matches(cli::TOKEN) {
        cli::token::run()?;
    } else if let Some(_) = cli.subcommand_matches(cli::LIST) {
        let config = Config::load()?;
        cli::list::run(config)?;
    }

    Ok(())
}
