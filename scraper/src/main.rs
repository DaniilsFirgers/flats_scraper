mod scrape_flats;
use scrape_flats::{districts, flats};
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    districts::parse_list_of_districts().await;
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
}
