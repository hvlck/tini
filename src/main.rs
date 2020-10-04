// std
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Error, Write};

// external
use clap::{App, Arg, SubCommand};
use scraper::{Html, Selector};
use serde_json::to_string_pretty;
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
        let mut index_items: Vec<String> = Vec::new();
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

                let body_selector = Selector::parse("body,body *").unwrap();
                let body_elements: Vec<_> = html.select(&body_selector).collect();
                let mut body_text = String::new();

                for element in body_elements.iter() {
                    let element_text: Vec<_> = element.text().collect();
                    body_text.push_str(element_text[0]);
                }

                let new_item = to_string_pretty(
                    &Item::new(
                        title[0],
                        &format!(
                            "{}/{}",
                            v.value_of("base").unwrap(),
                            file_path.path().strip_prefix("./").unwrap().display()
                        )[..],
                        &body_text,
                    )
                    .expect("Failed to parse url."),
                )
                .expect("Failed to index file.");

                index_items.push(new_item);
            }
        }

        let mut output_file = File::create("index.json").expect("Failed to generate index file!");
        write!(output_file, "{}", index_items.join("")).unwrap();
    }
}
