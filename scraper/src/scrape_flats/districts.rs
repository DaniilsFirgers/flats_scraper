// use reqwest::Error;

const MAIN_URL: &str = "https://www.ss.com/en/real-estate/flats/riga/";

pub async fn parse_list_of_districts() {
    let mut _res = reqwest::get(MAIN_URL);
}
