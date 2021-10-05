use std::{env, sync::Arc, time::Duration};

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
        let crawler = Crawler::new(Duration::from_millis(200), 2, 500);

        match spider_name {
            "cvedetails" => {
                let spider = Arc::new(spiders::cvedetails::CveDetailsSpider::new());
                crawler.run(spider);
            }
            "github" => {
                let spider = Arc::new(spiders::github::GitHubSpider::new());
                crawler.run(spider);
            }
            "google" => {
                let spider = Arc::new(spiders::google::GoogleSpider::new());
                crawler.run(spider);
            }
            _ => return Err(Error::InvalidSpider(spider_name.to_string()).into()),
        };
    }

    Ok(())
}

// use tokio::sync::Barrier;
// use std::sync::Arc;

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {

// let mut handles = Vec::with_capacity(10);
// let barrier = Arc::new(Barrier::new(10));
// for _ in 0..10 {
//     let c = barrier.clone();
//     // The same messages will be printed together.
//     // You will NOT see any interleaving.
//     handles.push(tokio::spawn(async move {
//         println!("before wait");
//         let wait_result = c.wait().await;
//         println!("after wait");
//         wait_result
//     }));
// }

// // Will not resolve until all "after wait" messages have been printed
// let mut num_leaders = 0;
// for handle in handles {
//     let wait_result = handle.await.unwrap();
//     if wait_result.is_leader() {
//         num_leaders += 1;
//     }
// }

// // Exactly one barrier will resolve as the "leader"
// assert_eq!(num_leaders, 1);

// Ok(())
// }
