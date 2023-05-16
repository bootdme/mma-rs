use reqwest::get;
use scraper::{Html, Selector};
use url::Url;
use serde_json::json;
use serde::Serialize;
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

    let wins_selector = Selector::parse(".wins").unwrap();
    if let Some(wins_element) = info_element.select(&wins_selector).next() {
        let wins_total_selector = Selector::parse(".win span:nth-child(2)").unwrap();
        if let Some(el) = wins_element.select(&wins_total_selector).next() {
            fighter.wins.total = el.text().collect::<String>().trim().parse::<u32>().unwrap_or(0);
        }

        let wins_by_selector = Selector::parse(".pl").unwrap();
        let win_by_elements: Vec<_> = wins_element.select(&wins_by_selector).collect();

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

    let losses_selector = Selector::parse(".loses").unwrap();
    if let Some(losses_element) = info_element.select(&losses_selector).next() {
        let losses_total_selector = Selector::parse(".lose span:nth-child(2)").unwrap();
        if let Some(el) = losses_element.select(&losses_total_selector).next() {
            fighter.losses.total = el.text().collect::<String>().trim().parse::<u32>().unwrap_or(0);
        }

        let losses_by_selector = Selector::parse(".pl").unwrap();
        let loss_by_elements: Vec<_> = losses_element.select(&losses_by_selector).collect();

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

    let no_contests_selector = Selector::parse(".nc span:nth-child(2)").unwrap();
    if let Some(no_contests_element) = info_element.select(&no_contests_selector).next() {
        fighter.no_contests = no_contests_element.text().collect::<String>().trim().parse().unwrap_or(0);
    }

    let fight_history_selector = Selector::parse(".module.fight_history tr:not(.table_head)").unwrap();
    let fights = document.select(&fight_history_selector);

    for fight_element in fights {
        let mut fight = Fight::default();

        let result_selector = Selector::parse("td:nth-child(1) .final_result").unwrap();
        if let Some(result_element) = fight_element.select(&result_selector).next() {
            fight.result = result_element.text().collect();
        }

        let opponent_name_selector = Selector::parse("td:nth-child(2) a").unwrap();
        if let Some(opponent_name_element) = fight_element.select(&opponent_name_selector).next() {
            fight.opponent = opponent_name_element.text().collect();
            if let Some(href_attr) = opponent_name_element.value().attr("href") {
                fight.opponent_url = href_attr.to_string();
            }
        }

        let event_selector = Selector::parse("td:nth-child(3) a").unwrap();
        if let Some(event_name_element) = fight_element.select(&event_selector).next() {
            fight.name = event_name_element.text().collect();
            if let Some(href_attr) = event_name_element.value().attr("href") {
                fight.event_url = href_attr.to_string();
            }
        }

        let event_date_selector = Selector::parse("td:nth-child(3) .sub_line").unwrap();
        if let Some(event_date_element) = fight_element.select(&event_date_selector).next() {
            fight.date = event_date_element.text().collect();
        }

        let method_selector = Selector::parse("td:nth-child(4)").unwrap();
        if let Some(method_element) = fight_element.select(&method_selector).next() {
            let method_text: String = method_element.text().collect();
            let method_parts: Vec<&str> = method_text.splitn(2, ')').collect();
            fight.method = format!("{}{}", method_parts[0], ')');
        }

        let referee_selector = Selector::parse("td:nth-child(4) .sub_line").unwrap();
        if let Some(referee_element) = fight_element.select(&referee_selector).next() {
            fight.referee = referee_element.text().collect();
        }

        let round_selector = Selector::parse("td:nth-child(5)").unwrap();
        if let Some(round_element) = fight_element.select(&round_selector).next() {
            fight.round = round_element.text().collect();
        }

        let time_selector = Selector::parse("td:nth-child(6)").unwrap();
        if let Some(time_element) = fight_element.select(&time_selector).next() {
            fight.time = time_element.text().collect();
        }

        if !fight.result.is_empty() {
            fighter.fights.push(fight);
        }
    }

    Ok(fighter)
}

#[tokio::main]
async fn main() {
    match get_sherdog_url("Jon Jones").await {
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
