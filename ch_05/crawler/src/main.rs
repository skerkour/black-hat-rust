use clap::{App, Arg, SubCommand};

mod crawler;
mod spiders;

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

    if let Some(_) = cli.subcommand_matches("spiders") {
        let spiders = spiders::all_spiders();
        // HashMap keys to vec
        let spider_names = spiders.keys().cloned().collect::<Vec<String>>();
        for name in spider_names {
            println!("{}", name);
        }
    } else if let Some(matches) = cli.subcommand_matches("run") {
        // we can safely unwrap as the argument is required
        let spider = matches.value_of("spider").unwrap();
    }

    Ok(())
}
