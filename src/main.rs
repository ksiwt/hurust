use clap::{Arg, Command};

fn main() {
    let matches = Command::new("hurust")
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
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let title = sub_matches.value_of("title").unwrap();
            if let Err(err) = hurust::run(title) {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
        _ => unreachable!("arg required"),
    }
}
