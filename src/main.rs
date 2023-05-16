mod fighter;
mod selectors;
mod sherdog;

use sherdog::{get_sherdog_url, get_fighter_data};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a fighter name.");
        return;
    }
    let fighter_name = &args[1];

    match get_sherdog_url(fighter_name).await {
        Ok(url) => {
            println!("Sherdog url is {}", url);
            match get_fighter_data(&url).await {
                Ok(fighter) => {
                    println!("Fighter data: {}", serde_json::to_string_pretty(&fighter).unwrap());
                }
                Err(e) => println!("An error occured while getting fighter data: {}", e),
            }
        }

        Err(e) => println!("An error occurred: {}", e),
    }
}
