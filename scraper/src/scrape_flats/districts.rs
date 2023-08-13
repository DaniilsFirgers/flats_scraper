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
    // Select the second tbody element (index 1)
    if let Some(tbody_element) = document.select(&selector).nth(1) {
        let tr_selector = Selector::parse("tr").unwrap();
        let tr_elements = tbody_element.select(&tr_selector).collect::<Vec<_>>();

        // Iterate through rows (tr elements) starting from index 1
        for (index, tr_element) in tr_elements.iter().enumerate() {
            let num_rows = tr_elements.len();
            if index == 0 || index == num_rows - 1 {
                continue; // Skip the first and last rows
            }
            let inner = tr_element.inner_html().to_string();
            println!("Some data: {}", &inner);
        }
    }
}
