fn main() {
    let cli = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name("list").about("List all activre agents"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::DisableVersion)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();
}
