mod scrape_flats;
use scrape_flats::{flats, utils};
use std::time::Instant;
use tokio;
use utils::read_user_input;

#[tokio::main]
async fn main() {
    let district_name = read_user_input();
    let start_time = Instant::now();
    flats::parse_flats(district_name).await;
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
}
