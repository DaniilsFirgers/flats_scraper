mod scrape_flats;
use districts::DISTRICTS;
use scrape_flats::{districts, flats, utils};
use tokio;
// use utils::read_user_input;

#[tokio::main]
async fn main() {
    for district in DISTRICTS.iter() {
        flats::parse_flats(district.to_string(), false).await;
        println!("Successfully scraped data for: {}", district.to_string());
    }
}
