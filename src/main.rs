use reqwest::get;
use scraper::{Html, Selector};
use url::Url;
use serde_json::json;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
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

#[derive(Default, Serialize)]
struct Fighter {
    url: String,
    name: String,
    nickname: String,
    age: String,
    birthday: String,
    locality: String,
    nationality: String,
    association: Vec<String>,
    height: String,
    weight: String,
    weight_class: String,
    image_url: String,
    wins: Wins,
    losses: Losses,
    no_contests: u32,
    fights: Vec<Fight>,
}

#[derive(Default, Serialize)]
struct Wins {
    total: u32,
    knockouts: u32,
    submissions: u32,
    decisions: u32,
    others: u32,
}

#[derive(Default, Serialize)]
struct Losses {
    total: u32,
    knockouts: u32,
    submissions: u32,
    decisions: u32,
    others: u32,
}

#[derive(Default, Serialize)]
struct Fight {
    name: String,
    date: String,
    opponent: String,
    result: String,
    method: String,
    referee: String,
    round: String,
    time: String,
    event_url: String,
    opponent_url: String,
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

async fn get_fighter_data(url: &str) -> Result<Fighter, Box<dyn Error>> {
    let body = get(url).await?.text().await?;
    let document = Html::parse_document(&body);
    let mut fighter: Fighter = Default::default();

    fighter.url = url.to_string();

    let info_selector = Selector::parse(".fighter-info").unwrap();
    let info_element = document.select(&info_selector).next().unwrap();

    let name_selector = Selector::parse("[itemprop='name'] > .fn").unwrap();
    if let Some(name_element) = info_element.select(&name_selector).next() {
        fighter.name = name_element.text().collect();
    }

    let nickname_selector = Selector::parse("[itemprop='name'] > .nickname").unwrap();
    if let Some(nickname_element) = info_element.select(&nickname_selector).next() {
        let nickname: String = nickname_element.text().collect();
        fighter.nickname = nickname[1..nickname.len() - 1].to_string();
    }

    let image_url_selector = Selector::parse("img.profile-image.photo").unwrap();
    if let Some(image_url_element) = info_element.select(&image_url_selector).next() {
        if let Some(src_attr) = image_url_element.value().attr("src") {
            fighter.image_url = src_attr.to_string();
        }
    }

    let age_selector = Selector::parse("[itemprop='birthDate']").unwrap();
    if let Some(age_element) = info_element.select(&age_selector).next() {
        fighter.age = age_element.text().collect();
    }

    let locality_selector = Selector::parse("[itemprop='addressLocality']").unwrap();
    if let Some(locality_element) = info_element.select(&locality_selector).next() {
        fighter.locality = locality_element.text().collect();
    }

    let nationality_selector = Selector::parse("strong[itemprop='nationality']").unwrap();
    if let Some(nationality_element) = info_element.select(&nationality_selector).next() {
        fighter.nationality = nationality_element.text().collect();
    }

    let height_selector = Selector::parse("[itemprop='height']").unwrap();
    if let Some(height_element) = info_element.select(&height_selector).next() {
        fighter.height = height_element.text().collect();
    }

    let weight_selector = Selector::parse("[itemprop='weight']").unwrap();
    if let Some(weight_element) = info_element.select(&weight_selector).next() {
        fighter.weight = weight_element.text().collect();
    }

    let association_selector = Selector::parse(".association > [itemprop='name']").unwrap();
    fighter.association = info_element.select(&association_selector)
        .map(|element| element.text().collect::<String>())
        .collect();

    let weight_class_selector = Selector::parse(".association-class > a").unwrap();
    if let Some(weight_class_element) = info_element.select(&weight_class_selector).next() {
        fighter.weight_class = weight_class_element.text().collect();
    }

    Ok(fighter)
}

#[tokio::main]
async fn main() {
    match get_sherdog_url("Felipe Fogolin").await {
        Ok(url) => {
            println!("Sherdog url is {}", url);
            match get_fighter_data(&url).await {
                Ok(fighter) => {
                    println!("Fighter data: {}", serde_json::to_string(&fighter).unwrap());
                }
                Err(e) => println!("An error occured while getting fighter data: {}", e),
            }
        }

        Err(e) => println!("An error occurred: {}", e),
    }
}
