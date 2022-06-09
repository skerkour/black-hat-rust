use clap::{Command, Arg};
use std::{env, sync::Arc, time::Duration};

mod crawler;
mod error;
mod spiders;

use crate::crawler::Crawler;
use error::Error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(Command::new("spiders").about("List all spiders"))
        .subcommand(
            Command::new("run").about("Run a spider").arg(
                Arg::new("spider")
                    .short('s')
                    .long("spider")
                    .help("The spider to run")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .arg_required_else_help(true)
        .get_matches();

    env::set_var("RUST_LOG", "info,crawler=debug");
    env_logger::init();

    if let Some(_) = cli.subcommand_matches("spiders") {
        let spider_names = vec!["cvedetails", "github", "quotes"];
        for name in spider_names {
            println!("{}", name);
        }
    } else if let Some(matches) = cli.subcommand_matches("run") {
        // we can safely unwrap as the argument is required
        let spider_name = matches.value_of("spider").unwrap();
        let crawler = Crawler::new(Duration::from_millis(200), 2, 500);

        match spider_name {
            "cvedetails" => {
                let spider = Arc::new(spiders::cvedetails::CveDetailsSpider::new());
                crawler.run(spider).await;
            }
            "github" => {
                let spider = Arc::new(spiders::github::GitHubSpider::new());
                crawler.run(spider).await;
            }
            "quotes" => {
                let spider = spiders::quotes::QuotesSpider::new().await?;
                let spider = Arc::new(spider);
                crawler.run(spider).await;
            }
            _ => return Err(Error::InvalidSpider(spider_name.to_string()).into()),
        };
    }

    Ok(())
}
