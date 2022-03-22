use clap::{Arg, Command};

pub fn parse() -> clap::ArgMatches {
    Command::new("hurust")
        .about("create new post with git checkout branch")
        .version("1.0.0")
        .author("mudrk")
        .subcommand_required(true)
        // Query subcommand
        .subcommand(
            Command::new("new")
                .short_flag('n')
                .long_flag("new")
                .about("create new post")
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("input post title")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches()
}
