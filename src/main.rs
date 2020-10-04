// std
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Error, Write};

// external
use clap::{App, Arg, SubCommand};
use scraper::{Html, Selector};
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
        let mut index_items: Vec<Item> = Vec::new();
        for entry in WalkDir::new(v.value_of("INPUT").unwrap_or("./")) {
            let file_path = entry.unwrap();
            if file_path
                .path()
                .extension()
                .unwrap_or(std::ffi::OsStr::new(""))
                != "html"
            {
                continue;
            } else {
                let html = Html::parse_document(
                    read_to_string(file_path.path())
                        .expect("Failed to read file!")
                        .as_str(),
                );
                let title_selector = Selector::parse("head title").unwrap();
                let title: Vec<_> = html
                    .select(&title_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect();
                println!("{} | Title: {:?}", file_path.path().display(), title[0]);
            }
        }

        let mut output_file = File::create("index.json").expect("Failed to generate index file!");
        write!(output_file, "{:?}", index_items.as_slice());
    }
}
