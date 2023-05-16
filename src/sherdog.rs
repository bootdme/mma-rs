use crate::selectors::*;
use crate::fighter::{Fight, Fighter};

use url::Url;
use std::fmt;
use std::error::Error;
use scraper::Html;
use reqwest::get;
use std::time::Instant;

#[derive(Debug)]
pub struct SherdogError {
    details: String
}

impl SherdogError {
    pub fn new(msg: &str) -> SherdogError {
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

pub async fn get_sherdog_url(fighter: &str) -> Result<String, Box<dyn Error>> {
    let search_url = format!("https://www.google.com/search?q={}%20sherdog", fighter);

    let resp = get(&search_url).await?.text().await?;

    let fragment = Html::parse_document(&resp);

    let selector = &SELECTOR;

    // Loop over each link in the HTML
    let result = fragment.select(selector).find_map(|element| {
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

pub async fn get_fighter_data(url: &str) -> Result<Fighter, Box<dyn Error>> {
    let start_time = Instant::now();
    let body = get(url).await?.text().await?;
    let document = Html::parse_document(&body);
    let mut fighter = Fighter { url: url.to_string(), ..Default::default() };

    let info_element = document.select(&INFO_SELECTOR).next().ok_or("Failed to parse info element")?;

    if let Some(name_element) = info_element.select(&NAME_SELECTOR).next() {
        fighter.name = name_element.text().collect();
    }

    if let Some(nickname_element) = info_element.select(&NICKNAME_SELECTOR).next() {
        let nickname: String = nickname_element.text().collect();
        fighter.nickname = nickname[1..nickname.len() - 1].to_string();
    }

    if let Some(image_url_element) = info_element.select(&IMAGE_URL_SELECTOR).next() {
        if let Some(src_attr) = image_url_element.value().attr("src") {
            fighter.image_url = src_attr.to_string();
        }
    }

    if let Some(age_element) = info_element.select(&AGE_SELECTOR).next() {
        fighter.age = age_element.text().collect();
    }

    if let Some(locality_element) = info_element.select(&LOCALITY_SELECTOR).next() {
        fighter.locality = locality_element.text().collect();
    }

    if let Some(nationality_element) = info_element.select(&NATIONALITY_SELECTOR).next() {
        fighter.nationality = nationality_element.text().collect();
    }

    if let Some(height_element) = info_element.select(&HEIGHT_SELECTOR).next() {
        fighter.height = height_element.text().collect();
    }

    if let Some(weight_element) = info_element.select(&WEIGHT_SELECTOR).next() {
        fighter.weight = weight_element.text().collect();
    }

    fighter.association = info_element.select(&ASSOCIATION_SELECTOR)
        .map(|element| element.text().collect::<String>())
        .collect();

    if let Some(weight_class_element) = info_element.select(&WEIGHT_CLASS_SELECTOR).next() {
        fighter.weight_class = weight_class_element.text().collect();
    }

    if let Some(wins_element) = info_element.select(&WINS_SELECTOR).next() {
        if let Some(el) = wins_element.select(&WINS_TOTAL_SELECTOR).next() {
            fighter.wins.total = el.text().collect::<String>().trim().parse::<u32>().unwrap_or(0);
        }

        let win_by_elements: Vec<_> = wins_element.select(&WINS_BY_SELECTOR).collect();

        if let Some(el) = win_by_elements.get(0) {
            fighter.wins.knockouts = el.text().collect::<String>().trim().parse().unwrap_or(0);
        }

        if let Some(el) = win_by_elements.get(1) {
            fighter.wins.submissions = el.text().collect::<String>().trim().parse().unwrap_or(0);
        }

        if let Some(el) = win_by_elements.get(2) {
            fighter.wins.decisions = el.text().collect::<String>().trim().parse().unwrap_or(0);
        }

        if let Some(el) = win_by_elements.get(3) {
            fighter.wins.others = el.text().collect::<String>().trim().parse().unwrap_or(0);
        }
    }

    if let Some(losses_element) = info_element.select(&LOSSES_SELECTOR).next() {
        if let Some(el) = losses_element.select(&LOSSES_TOTAL_SELECTOR).next() {
            fighter.losses.total = el.text().collect::<String>().trim().parse::<u32>().unwrap_or(0);
        }

        let loss_by_elements: Vec<_> = losses_element.select(&LOSSES_BY_SELECTOR).collect();

        if let Some(el) = loss_by_elements.get(0) {
            fighter.losses.knockouts = el.text().collect::<String>().trim().parse().unwrap_or(0);
        }

        if let Some(el) = loss_by_elements.get(1) {
            fighter.losses.submissions = el.text().collect::<String>().trim().parse().unwrap_or(0);
        }

        if let Some(el) = loss_by_elements.get(2) {
            fighter.losses.decisions = el.text().collect::<String>().trim().parse().unwrap_or(0);
        }

        if let Some(el) = loss_by_elements.get(3) {
            fighter.losses.others = el.text().collect::<String>().trim().parse().unwrap_or(0);
        }
    }

    if let Some(no_contests_element) = info_element.select(&NO_CONTESTS_SELECTOR).next() {
        fighter.no_contests = no_contests_element.text().collect::<String>().trim().parse().unwrap_or(0);
    }

    let fights = document.select(&FIGHT_HISTORY_SELECTOR);

    for fight_element in fights {
        let mut fight = Fight::default();

        if let Some(result_element) = fight_element.select(&RESULT_SELECTOR).next() {
            fight.result = result_element.text().collect();
        }

        if let Some(opponent_name_element) = fight_element.select(&OPPONENT_NAME_SELECTOR).next() {
            fight.opponent = opponent_name_element.text().collect();
            if let Some(href_attr) = opponent_name_element.value().attr("href") {
                fight.opponent_url = href_attr.to_string();
            }
        }

        if let Some(event_name_element) = fight_element.select(&EVENT_SELECTOR).next() {
            fight.name = event_name_element.text().collect();
            if let Some(href_attr) = event_name_element.value().attr("href") {
                fight.event_url = href_attr.to_string();
            }
        }

        if let Some(event_date_element) = fight_element.select(&EVENT_DATE_SELECTOR).next() {
            fight.date = event_date_element.text().collect();
        }

        if let Some(method_element) = fight_element.select(&METHOD_SELECTOR).next() {
            let method_text: String = method_element.text().collect();
            let method_parts: Vec<&str> = method_text.splitn(2, ')').collect();
            fight.method = format!("{}{}", method_parts[0], ')');
        }

        if let Some(referee_element) = fight_element.select(&REFEREE_SELECTOR).next() {
            fight.referee = referee_element.text().collect();
        }

        if let Some(round_element) = fight_element.select(&ROUND_SELECTOR).next() {
            fight.round = round_element.text().collect();
        }

        if let Some(time_element) = fight_element.select(&TIME_SELECTOR).next() {
            fight.time = time_element.text().collect();
        }

        if !fight.result.is_empty() {
            fighter.fights.push(fight);
        }
    }

    let elapsed = start_time.elapsed();
    println!("Scraped page in {:.2?}", elapsed);

    Ok(fighter)
}
