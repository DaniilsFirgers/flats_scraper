mod scrape_flats;
use scrape_flats::{districts, flats};
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() {
    districts::DISTRICTS.iter().for_each(|district| {
        println!("{}", district);
    });
    let start_time = Instant::now();
    flats::parse_flats().await;
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
}
