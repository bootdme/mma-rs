use serde::Serialize;

/// Represents a fighter's profile and statistics.
/// All strings are default to an empty string, and integers default to zero.
/// Fights are represented as a vector of `Fight` structs.
#[derive(Default, Serialize)]
pub struct Fighter {
    pub url: String,
    pub name: String,
    pub nickname: String,
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
    pub no_contests: u8,
    pub fights: Vec<Fight>,
}

/// Represents a fighter's wins statistics.
/// All fields default to zero.
#[derive(Default, Serialize)]
pub struct Wins {
    pub total: u8,
    pub knockouts: u8,
    pub submissions: u8,
    pub decisions: u8,
    pub others: u8,
}

/// Represents a fighter's losses statistics.
/// All fields default to zero.
#[derive(Default, Serialize)]
pub struct Losses {
    pub total: u8,
    pub knockouts: u8,
    pub submissions: u8,
    pub decisions: u8,
    pub others: u8,
}

/// Represents a single fight in a fighter's fight history.
/// All strings default to an empty string.
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
