use clap::{App, SubCommand};

mod api;
mod cli;
mod config;
mod error;

pub use error::Error;

fn main() -> Result<(), anyhow::Error> {
    let cli = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name(cli::AGENTS).about("List all agents"))
        .subcommand(SubCommand::with_name(cli::EXEC).about("Execute a command"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::DisableVersion)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    let api_client = api::Client::new(config::SERVER_URL.to_string());

    if let Some(_) = cli.subcommand_matches(cli::AGENTS) {
        cli::agents::run(&api_client)?;
    } else if let Some(_) = cli.subcommand_matches(cli::EXEC) {
        cli::exec::run(&api_client)?;
    }

    Ok(())
}
