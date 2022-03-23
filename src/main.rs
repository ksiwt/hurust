mod cli;
mod hugo;

use git2::Error;

fn run() -> Result<(), Error> {
    let matches = cli::parse();
    if let Some(matches) = matches.subcommand_matches("new") {
        let title = matches.value_of("title").unwrap();
        hurust::run(title)?;
        hugo::create_post(title);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
