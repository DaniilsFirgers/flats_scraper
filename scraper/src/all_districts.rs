mod scrape_flats;
use scrape_flats::{flats, utils, districts};
use std::time::Instant;
use tokio;
use utils::read_user_input;
use districts::DISTRICTS;

#[tokio::main]
async fn main() {
    // for district in DISTRICTS.iter() {
    //         flats::parse_flats(district.to_string(), false).await;
    //         println!("Successfully scraped data for: {}", district.to_string());
    // }

    flats::parse_flats("other".to_string(), false).await;
}
