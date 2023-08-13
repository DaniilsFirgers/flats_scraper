use reqwest::StatusCode;

const MAIN_URL: &str = "https://www.ss.com/en/real-estate/flats/riga/all/";

pub async fn parse_list_of_districts() {
    let client = reqwest::Client::new();
    let mut res = client.get(MAIN_URL).send().await.unwrap();

    let raw_html = match res.status() {
        StatusCode::OK => res.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    println!("HTML: {}", raw_html);
}
