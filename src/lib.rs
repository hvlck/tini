// std
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Write};
use std::result::Result;

// external
use rayon::prelude::*; // glob import for now
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::to_vec_pretty;
use url::{ParseError, Url};
use walkdir::WalkDir;

/// Represents an index of items
#[derive(Serialize, Debug, Deserialize)]
pub struct Index {
    pub items: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents an item (page) in the tinysearch index
pub struct Item {
    /// The title of the page
    pub title: String,
    /// The url of the page (base -b arg combined with the route)
    pub url: String,
    /// The text of the page
    pub body: String,
}

impl Item {
    pub fn new<T>(title: T, url: &str, body: T) -> Result<Item, ParseError>
    where
        T: Into<String>,
    {
        let parsed_url = Url::parse(url)?;

        Ok(Item {
            title: title.into(),
            url: parsed_url.to_string(),
            body: body.into(),
        })
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{
            title: {},
            url: {},
            body: {}
        }}",
            self.title, self.url, self.body
        )
    }
}

pub fn index(input: &str, base: &str) -> Result<(), std::io::Error> {
    let file_index: Vec<_> = WalkDir::new(input).into_iter().collect();
    let file_index_len = file_index.len();

    let items: Vec<Vec<u8>> = file_index
        .into_par_iter()
        .enumerate()
        .map(|(i, entry)| {
            let file_path = entry.expect("Couldn't read file, aborting...");
            let mut new_item: Vec<u8> = Vec::new();

            if file_path
                .path()
                .extension()
                .unwrap_or(std::ffi::OsStr::new(""))
                == "html"
            {
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

                new_item = to_vec_pretty(
                    &Item::new(
                        title[0],
                        &format!(
                            "{}/{}",
                            base,
                            file_path
                                .path()
                                .strip_prefix("./")
                                .unwrap_or(file_path.path())
                                .display()
                        )[..],
                        &body_text,
                    )
                    .expect("Failed to parse URL."),
                )
                .expect("Failed to index file.");
            }

            if new_item.len() > 0 && i != file_index_len - 1 {
                [new_item, ",".as_bytes().to_vec()].concat()
            } else {
                new_item
            }
        })
        .collect();

    let idx = Index { items };

    let output_file = File::create("index.json").expect("Failed to generate index file!");
    let mut stream = BufWriter::new(output_file);

    stream.write(&"[".as_bytes()).unwrap();
    for i in 0..idx.items.len() {
        stream.write(&idx.items[i]).unwrap();
    }
    stream.write(&"]".as_bytes()).unwrap();

    stream.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir, remove_dir_all, write};
    use std::path::Path;
    #[test]
    fn test_new_item() {
        assert_eq!(Item::new("Example Title", "https://example.com/route", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.").unwrap().url, "https://example.com/route");
    }

    #[test]
    fn test_indexing() {
        create_dir("index_tests/").unwrap();
        for i in 0..100 {
            let num = i * (i + i);
            write(
                Path::new(&format!("index_tests/{}.html", i)),
                format!(
                    "
                <!DOCTYPE html>
                <html lang=\"en\">
                <head>
                    <meta charset=\"UTF-8\">
                    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
                    <title>Document</title>
                </head>
                <body>
                    {}{}
                </body>
                </html>
            ",
                    i, num
                ),
            )
            .unwrap();
        }

        index("index_tests/", "https://example.com").unwrap();
        remove_dir_all("index_tests/").unwrap();
    }
}
