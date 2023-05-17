mod fighter;
mod selectors;
mod sherdog;

use sherdog::{get_fighter_data, get_sherdog_url};

/// This is the main entry point for our application. The program
/// accepts command-line arguments, the first of which should be
/// a fighter's name.
///
/// # Examples
///
/// ```
/// $ cargo run --release "Conor McGregor"
/// ```
///
/// This will print the Sherdog URL of Conor McGregor and his fight history.
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
                    println!(
                        "Fighter data: {}",
                        serde_json::to_string_pretty(&fighter).unwrap()
                    );
                }
                Err(e) => println!("An error occured while getting fighter data: {}", e),
            }
        }

        Err(e) => println!("An error occurred: {}", e),
    }
}
