use clap::{App, Arg, SubCommand};

mod api;
mod cli;
mod config;
mod error;

pub use error::Error;

use crate::config::Config;

fn main() -> Result<(), anyhow::Error> {
    let cli = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name(cli::AGENTS).about("List all agents"))
        .subcommand(SubCommand::with_name(cli::IDENTITY).about("Generates a new identity keypair"))
        .subcommand(
            SubCommand::with_name(cli::EXEC)
                .about("Execute a command")
                .arg(
                    Arg::with_name("agent")
                        .short("a")
                        .long("agent")
                        .help("The agent id to execute the command on")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("command")
                        .help("The command to execute, with its arguments.")
                        .required(true)
                        .index(1),
                ),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    let api_client = api::Client::new(config::SERVER_URL.to_string());

    if let Some(_) = cli.subcommand_matches(cli::AGENTS) {
        cli::agents::run(&api_client)?;
    } else if let Some(_) = cli.subcommand_matches(cli::IDENTITY) {
        cli::identity::run();
    } else if let Some(matches) = cli.subcommand_matches(cli::EXEC) {
        // we can sfaely unwrap as the arguments are required
        let agent_id = matches.value_of("agent").unwrap();
        let command = matches.value_of("command").unwrap();
        let conf = Config::load()?;
        cli::exec::run(&api_client, agent_id, command, conf)?;
    }

    Ok(())
}
