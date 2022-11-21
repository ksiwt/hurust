use clap::{Arg, Command};

pub fn parse() -> clap::ArgMatches {
    Command::new("Hurust")
        .about("Command line tool which act as wrapper of hugo new command")
        .version("1.0.0")
        .author("pohe")
        .subcommand_required(true)
        // Query subcommand
        .subcommand(
            Command::new("new")
                .short_flag('n')
                .long_flag("new")
                .about("create new content")
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("input content title")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches()
}
