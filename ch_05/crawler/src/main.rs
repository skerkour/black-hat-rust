use std::{env, time::Duration};

use clap::{App, Arg, SubCommand};

mod crawler;
mod error;
mod spiders;

use error::Error;

use crate::crawler::Crawler;

fn main() -> Result<(), anyhow::Error> {
    let cli = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name("spiders").about("List all spiders"))
        .subcommand(
            SubCommand::with_name("run").about("Run a spider").arg(
                Arg::with_name("spider")
                    .short("s")
                    .long("spider")
                    .help("The spider to run")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    if let Some(_) = cli.subcommand_matches("spiders") {
        let spider_names = vec!["cvedetails", "github", "google"];
        for name in spider_names {
            println!("{}", name);
        }
    } else if let Some(matches) = cli.subcommand_matches("run") {
        // we can safely unwrap as the argument is required
        let spider_name = matches.value_of("spider").unwrap();
        let crawler = Crawler::new(Duration::from_millis(125), 2, 500);

        match spider_name {
            "cvedetails" => {
                let spider = spiders::cvedetails::CveDetailsSpider::new();
                crawler.run(&spider);
            }
            "github" => {
                let spider = spiders::github::GitHubSpider::new();
                crawler.run(&spider);
            }
            "google" => {
                let spider = spiders::google::GoogleSpider::new();
                crawler.run(&spider);
            }
            _ => return Err(Error::InvalidSpider(spider_name.to_string()).into()),
        };
    }

    Ok(())
}
