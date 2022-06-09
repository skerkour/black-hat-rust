use clap::{Arg, Command};

mod api;
mod cli;
mod config;
mod error;

pub use error::Error;

fn main() -> Result<(), anyhow::Error> {
    let cli = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(Command::new(cli::AGENTS).about("List all agents"))
        .subcommand(Command::new(cli::JOBS).about("List all jobs"))
        .subcommand(
            Command::new(cli::EXEC)
                .about("Execute a command")
                .arg(
                    Arg::new("agent")
                        .short('a')
                        .long("agent")
                        .help("The agent id to execute the command on")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("command")
                        .help("The command to execute, with its arguments.")
                        .required(true)
                        .index(1),
                ),
        )
        .arg_required_else_help(true)
        .get_matches();

    let api_client = api::Client::new(config::SERVER_URL.to_string());

    if let Some(_) = cli.subcommand_matches(cli::AGENTS) {
        cli::agents::run(&api_client)?;
    } else if let Some(_) = cli.subcommand_matches(cli::JOBS) {
        cli::jobs::run(&api_client)?;
    } else if let Some(matches) = cli.subcommand_matches(cli::EXEC) {
        // we can sfaely unwrap as the arguments are required
        let agent_id = matches.value_of("agent").unwrap();
        let command = matches.value_of("command").unwrap();
        cli::exec::run(&api_client, agent_id, command)?;
    }

    Ok(())
}
