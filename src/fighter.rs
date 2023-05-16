use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Fighter {
    pub url: String,
    pub name: String,
    pub nickname: String,
    pub age: String,
    pub birthday: String,
    pub locality: String,
    pub nationality: String,
    pub association: Vec<String>,
    pub height: String,
    pub weight: String,
    pub weight_class: String,
    pub image_url: String,
    pub wins: Wins,
    pub losses: Losses,
    pub no_contests: u32,
    pub fights: Vec<Fight>,
}

#[derive(Default, Serialize)]
pub struct Wins {
    pub total: u32,
    pub knockouts: u32,
    pub submissions: u32,
    pub decisions: u32,
    pub others: u32,
}

#[derive(Default, Serialize)]
pub struct Losses {
    pub total: u32,
    pub knockouts: u32,
    pub submissions: u32,
    pub decisions: u32,
    pub others: u32,
}

#[derive(Default, Serialize)]
pub struct Fight {
    pub name: String,
    pub date: String,
    pub opponent: String,
    pub result: String,
    pub method: String,
    pub referee: String,
    pub round: String,
    pub time: String,
    pub event_url: String,
    pub opponent_url: String,
}
