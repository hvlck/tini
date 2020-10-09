// std
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Error, Write};
use std::time::Instant;

// external
use clap::{App, Arg, SubCommand};
use scraper::{Html, Selector};
use serde_json::to_vec_pretty;
use walkdir::WalkDir;

// local
use tini::Item;

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
                ),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("index") {
        let timing = Instant::now();

        let mut index_items = Vec::new();

        for entry in WalkDir::new(v.value_of("INPUT").unwrap_or("./")) {
            let file_path = entry.expect("Couldn't read file, aborting...");
            if file_path
                .path()
                .extension()
                .unwrap_or(std::ffi::OsStr::new(""))
                != "html"
            {
                continue;
            } else {
                // parse html document, convert it so it can be query-selectored-ed
                let html = Html::parse_document(
                    read_to_string(file_path.path())
                        .expect("Failed to read file!")
                        .as_str(),
                );

                let title_selector =
                    Selector::parse("head title").expect("Failed to read HTML title, aborting...");
                let title: Vec<_> = html
                    .select(&title_selector)
                    .next()
                    .expect("Failed to get HTML title, aborting...")
                    .text()
                    .collect();

                let body_selector =
                    Selector::parse("body,body *").expect("Failed to read HTML body, aborting...");
                let body_elements: Vec<_> = html.select(&body_selector).collect();
                let mut body_text = String::new();

                for element in body_elements.iter() {
                    let element_text: Vec<_> = element.text().collect();
                    if element_text.len() != 0 {
                        body_text.push_str(element_text[0]);
                    }
                }

                let new_item = to_vec_pretty(
                    &Item::new(
                        title[0],
                        &format!(
                            "{}/{}",
                            v.value_of("base").unwrap(),
                            file_path.path().strip_prefix("./").unwrap().display()
                        )[..],
                        &body_text,
                    )
                    .expect("Failed to parse URL."),
                )
                .expect("Failed to index file.");

                index_items.push(new_item);
            }
        }

        let output_file = File::create("index.json").expect("Failed to generate index file!");
        let mut stream = BufWriter::new(output_file);

        for i in 0..index_items.len() {
            stream.write(&index_items[i]).unwrap();
        }

        stream
            .flush()
            .expect("Failed to write to index file, aborting...");

        println!(
            "Tini: Completed indexing in {}ms",
            timing.elapsed().as_millis()
        );
    }
}
