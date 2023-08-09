mod scrape_flats;
use scrape_flats::{districts, flats};
use tokio;

#[tokio::main]
async fn main() {
    districts::parse_list_of_districts().await;
    println!("Hello, world!");
}
