// std

// external
use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new("tini")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate tinysearch index.")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(Arg::with_name("INPUT").help("Directory to index.")),
        )
        .get_matches();

    if let Some(_v) = app.subcommand_matches("generate") {
        println!("Called generate.");
    }
}
