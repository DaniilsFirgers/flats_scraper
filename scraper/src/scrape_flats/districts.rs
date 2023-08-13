use reqwest::StatusCode;
use scraper::{Html, Selector};

const MAIN_URL: &str = "https://www.ss.com/en/real-estate/flats/riga/centre/";

pub async fn parse_list_of_districts() {
    let client = reqwest::Client::new();
    let mut res = client.get(MAIN_URL).send().await.unwrap();

    let raw_html = match res.status() {
        StatusCode::OK => res.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };
    let document = Html::parse_document(&raw_html);
    let selector = Selector::parse("form#filter_frm>table>tbody").unwrap();
    for (index, element) in document.select(&selector).enumerate() {
        if index == 0 {
            continue;
        }
        let inner = element.inner_html().to_string();
        println!("Some shit {}", &inner);
    }
}
