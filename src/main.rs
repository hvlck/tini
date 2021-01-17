// std
use std::time::Instant;

// external
use clap::{App, Arg, SubCommand};

// local
use tini::index;

fn main() {
    // initializes clap, for CLI arg parsing
    let app = App::new("tini")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("index")
                .about("Generate tinysearch index.")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(Arg::with_name("INPUT").help("Directory to index."))
                .arg(
                    Arg::with_name("base")
                        .short("b")
                        .takes_value(true)
                        .help("The basic URL to add to all url fields.")
                        .required(true),
                ).arg(
                    Arg::with_name("ignore-body").short("i").long("ignore-body").help("If passed, the body of the file will not be included in the final tinysearch index.").required(false).takes_value(false)
                ),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("index") {
        let timing = Instant::now();

        index(
            v.value_of("INPUT").unwrap_or("./"),
            v.value_of("base").expect("Failed to provide a base URL."),
            v.is_present("ignore-body"),
        )
        .expect("Failed to write to index file, aborting...");

        println!(
            "Tini: Completed indexing in {}ms",
            timing.elapsed().as_millis()
        );
    }
}
