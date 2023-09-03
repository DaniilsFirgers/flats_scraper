mod scrape_flats;
use scrape_flats::{flats, utils};
use std::time::Instant;
use tokio;
use utils::read_user_input;

#[tokio::main]
async fn main() {
    let district_name = read_user_input();
    flats::parse_flats(district_name).await;
    println!("Successfully scraped data for: {}", district_name);
}
