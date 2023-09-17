mod scrape_flats;
use districts::DISTRICTS;
use scrape_flats::{districts, flats, utils};
use std::time::Instant;
use tokio;
use utils::read_user_input;

#[tokio::main]
async fn main() {
    for district in DISTRICTS.iter() {
        flats::parse_flats(district.to_string(), true).await;
        println!("Successfully scraped data for: {}", district.to_string());
    }
    // let district_name = read_user_input();
    // flats::parse_flats(district_name, true).await;
    // println!("Successfully scraped data for: {}", district_name);
}
