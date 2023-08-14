use super::structs::Flat;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::fs::File;
use std::fs::{self};
use std::io::{BufWriter, Write};

const MAIN_URL: &str = "https://www.ss.com/en/real-estate/flats/riga/centre/sell/";

pub async fn parse_list_of_districts() {
    let mut flats: Vec<Flat> = Vec::new();
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

            // Select the <td> elements inside the current <tr>
            let td_selector = Selector::parse("td").unwrap();

            let mut flat: Flat = Flat::new();

            for (index, td_element) in tr_element.select(&td_selector).enumerate() {
                // Check if the <td> element contains an <a> element
                if index == 0 || index == 1 {
                    continue;
                }
                let field = td_element
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string()
                    .replace("\n", ".");

                if index == 2 {
                    flat.set_description(field);
                } else if index == 3 {
                    flat.set_street(field);
                } else if index == 4 {
                    flat.set_rooms(field);
                } else if index == 5 {
                    flat.set_square(field);
                } else if index == 6 {
                    flat.set_floor(field);
                } else if index == 7 {
                    flat.set_series(field);
                } else if index == 8 {
                    flat.set_square_m_price(field);
                } else if index == 9 {
                    flat.set_price(field);
                }
            }
            flats.push(flat);
            println!("{:?}", flats);
        }
    }
    let saved_json = save_to_local_json_file(flats);
    match saved_json {
        Ok(_) => println!("Saved to local json file"),
        Err(e) => println!("Error saving to local json file: {}", e),
    }
}

fn save_to_local_json_file(flat_data: Vec<Flat>) -> Result<(), Box<dyn std::error::Error>> {
    // Create the directory if it doesn't exist
    fs::create_dir_all("scraper/data")?;
    let file = File::create("scraper/data/data.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &flat_data)?;
    writer.flush()?;
    Ok(())
}
