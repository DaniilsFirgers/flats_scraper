use super::structs::Flat;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::fs::File;
use std::fs::{self};
use std::io::{BufWriter, Write};

pub async fn parse_flats() {
    let mut last_page: i32 = 0;
    let mut page_number = 1;
    let mut reached_end = false;
    let client = reqwest::Client::new();
    let mut flats: Vec<Flat> = Vec::new();

    loop {
        if reached_end {
            break;
        }
        let base_url: &str = "https://www.ss.com/en/real-estate/flats/riga/centre/sell/page";

        let main_url = format!("{}{}.html", base_url, page_number);
        // println!("Page number: {} - {}", page_number, last_page);
        let mut res = client.get(main_url).send().await.unwrap();

        let raw_html = match res.status() {
            StatusCode::OK => res.text().await.unwrap(),
            _ => panic!("Something went wrong"),
        };
        let document = Html::parse_document(&raw_html);
        let main_table_selector = Selector::parse("form#filter_frm>table>tbody").unwrap();
        let main_pages_selector = Selector::parse("form#filter_frm div.td2").unwrap();
        let individual_page_index_selector = Selector::parse("a").unwrap();

        let first_main_pages_selector = document.select(&main_pages_selector).next().unwrap();

        // Iterate over all the pages
        let mut pages_iterator = first_main_pages_selector
            .select(&individual_page_index_selector)
            .enumerate()
            .peekable();

        let mut pen_ultimate_page: i32 = 0;

        while let Some((index, page)) = pages_iterator.next() {
            if index == 0 {
                continue; // Skip the first iteration
            }

            let page_string: String = page.text().collect::<String>();

            let page_numeric = match parse_string_to_int(page_string) {
                Ok(value) => value,
                Err(_) => pen_ultimate_page,
            };

            if pages_iterator.peek().is_none() {
                if pen_ultimate_page < last_page {
                    reached_end = true;
                    break;
                }
                last_page = pen_ultimate_page;
                break; // Stop the loop before the last iteration
            }
            pen_ultimate_page = page_numeric;
        }
        // Select the second tbody element (index 1)
        if let Some(tbody_element) = document.select(&main_table_selector).nth(1) {
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
            }
        }
        page_number += 1;
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

fn parse_string_to_int(string_value: String) -> Result<i32, Box<dyn std::error::Error>> {
    let parsed_i32: i32 = string_value.parse()?;

    Ok(parsed_i32)
}
