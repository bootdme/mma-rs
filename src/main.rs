use reqwest::get;
use scraper::{Html, Selector};
use url::Url;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct SherdogError {
    details: String
}

impl SherdogError {
    fn new(msg: &str) -> SherdogError {
        SherdogError{details: msg.to_string()}
    }
}

impl fmt::Display for SherdogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SherdogError {
    fn description(&self) -> &str {
        &self.details
    }
}

async fn get_sherdog_url(fighter: &str) -> Result<String, Box<dyn Error>> {
    let search_url = format!("https://www.google.com/search?q={}%20sherdog", fighter);

    let resp = get(&search_url).await?.text().await?;

    let fragment = Html::parse_document(&resp);

    let selector = Selector::parse("a").expect("Failed to create a Selector");

    // Loop over each link in the HTML
    let result = fragment.select(&selector).find_map(|element| {
        // Check if the href attribute exists
        if let Some(href) = element.value().attr("href") {
            // If the link contains 'sherdog.com/fighter', return it
            if href.contains("sherdog.com/fighter") {
                let full_url = format!("https://www.google.com{}", href);
                let parsed_url = Url::parse(&full_url).expect("Failed to parse URL");
                parsed_url.query_pairs().find(|(k, _)| k == "q").map(|(_, v)| v.into_owned())
            } else {
                None
            }
        } else {
            None
        }
    });

    // If no link found, return an error
    match result {
        Some(url) => Ok(url),
        None => Err(Box::new(SherdogError::new("No sherdog link found"))),
    }
}

#[tokio::main]
async fn main() {
    match get_sherdog_url("The Diamond").await {
        Ok(url) => println!("Sherdog url is {}", url),
        Err(e) => println!("An error occurred: {}", e),
    }
}
