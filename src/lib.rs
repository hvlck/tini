use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub title: String,
    pub url: String,
    pub body: String,
}

impl Item {
    pub fn new<T>(title: T, url: &str, body: T) -> std::result::Result<Item, ParseError>
    where
        T: Into<String> + Clone,
    {
        let parsed_url = Url::parse(url)?;

        Ok(Item {
            title: title.into(),
            url: parsed_url.to_string(),
            body: body.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_item() {
        assert_eq!(Item::new("Example Title", "https://example.com/route", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.").unwrap().url, "https://example.com/route");
    }
}
