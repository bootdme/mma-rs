# Sherdog MMA Web Scraper in Rust

A version of [woog2roid's](https://github.com/woog2roid/mma-api) MMA API in Rust!

## Installation

1. Run `git clone https://github.com/bootdme/mma-rs`
2. Run `cargo build --release` to install dependencies
3. Run `cargo run --release "fighter-name"` to retrieve fighter information

## Example

`cargo run --release "Bo Nickal"`

```json
{
  "url": "https://www.sherdog.com/fighter/Bo-Nickal-392031",
  "name": "Bo Nickal",
  "nickname": "",
  "birthday": "Jan 14, 1996",
  "locality": "State College, Pennsylvania",
  "nationality": "United States",
  "association": [
    "American Top Team Happy Valley"
  ],
  "height": "6'1\"",
  "weight": "185 lbs",
  "weight_class": "Middleweight",
  "image_url": "/image_crop/200/300/_images/fighter/20220929071926_Bo_Nickal_ff.JPG",
  "wins": {
    "total": 4,
    "knockouts": 1,
    "submissions": 3,
    "decisions": 0,
    "others": 0
  },
  "losses": {
    "total": 0,
    "knockouts": 0,
    "submissions": 0,
    "decisions": 0,
    "others": 0
  },
  "no_contests": 0,
  "fights": [
    {
      "name": "UFC 285 - Jones vs. Gane",
      "date": "Mar / 04 / 2023",
      "opponent": "Jamie Pickett",
      "result": "win",
      "method": "Submission (Arm-Triangle Choke)",
      "referee": "Keith Peterson",
      "round": "1",
      "time": "2:54",
      "event_url": "/events/UFC-285-Jones-vs-Gane-95232",
      "opponent_url": "/fighter/Jamie-Pickett-72595"
    },
    {
      "name": "Dana White's Contender Series - Contender Series 2022: Week 10",
      "date": "Sep / 27 / 2022",
      "opponent": "Donovan Beard",
      "result": "win",
      "method": "Submission (Triangle Choke)",
      "referee": "Mark Smith",
      "round": "1",
      "time": "0:52",
      "event_url": "/events/Dana-Whites-Contender-Series-Contender-Series-2022-Week-10-93236",
      "opponent_url": "/fighter/Donovan-Beard-310501"
    },
    {
      "name": "Dana White's Contender Series - Contender Series 2022: Week 3",
      "date": "Aug / 09 / 2022",
      "opponent": "Zachary Borrego",
      "result": "win",
      "method": "Submission (Rear-Naked Choke)",
      "referee": "Mike Beltran",
      "round": "1",
      "time": "1:02",
      "event_url": "/events/Dana-Whites-Contender-Series-Contender-Series-2022-Week-3-93229",
      "opponent_url": "/fighter/Zachary-Borrego-383740"
    },
    {
      "name": "iFC 3 - Jorge Masvidal's iKon Fighting Championship 3",
      "date": "Jun / 03 / 2022",
      "opponent": "John Noland",
      "result": "win",
      "method": "KO (Punches)",
      "referee": "Mike King",
      "round": "1",
      "time": "0:33",
      "event_url": "/events/iFC-3-Jorge-Masvidals-iKon-Fighting-Championship-3-93175",
      "opponent_url": "/fighter/John-Noland-245673"
    },
    {
      "name": "Square Ring Promotions - Island Fights 70",
      "date": "Nov / 05 / 2021",
      "opponent": "Billy Goode",
      "result": "win",
      "method": "KO (Punch)",
      "referee": "",
      "round": "1",
      "time": "0:56",
      "event_url": "/events/Square-Ring-Promotions-Island-Fights-70-91018",
      "opponent_url": "/fighter/Billy-Goode-378712"
    },
    {
      "name": "Square Ring Promotions - Island Fights 69",
      "date": "Sep / 24 / 2021",
      "opponent": "David Conley",
      "result": "win",
      "method": "Submission (Guillotine Choke)",
      "referee": "",
      "round": "1",
      "time": "2:02",
      "event_url": "/events/Square-Ring-Promotions-Island-Fights-69-91019",
      "opponent_url": "/fighter/David-Conley-386727"
    }
  ]
}
```
