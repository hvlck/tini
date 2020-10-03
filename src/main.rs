// std

// external
use clap::{App, Arg, SubCommand};
use walkdir::WalkDir;

// local
use tini::Item;

fn main() {
    let app = App::new("tini")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("index")
                .about("Generate tinysearch index.")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(Arg::with_name("INPUT").help("Directory to index.")),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("index") {
        for entry in WalkDir::new(v.value_of("INPUT").unwrap_or("./")) {
            println!("{}", entry.unwrap().path().display());
        }
    }
}
